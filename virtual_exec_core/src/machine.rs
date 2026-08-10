use crate::HashMap;
use crate::fn_extern::{FnExtern, MethodResolver};
use crate::sequential::exec::{FnStackFrame, InstStateMachine, State};
use crate::sequential::instructions::Instruction;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use async_lock::RwLock;
use cfg_if::cfg_if;
use virtual_exec_parser::error::ParseError;
use virtual_exec_parser::parser::parse_expr;
use virtual_exec_type::ast::core::Module;
use virtual_exec_type::error::{CriticalError, ExecutionError, MemoryError, RecoverableError};
use virtual_exec_type::ext::*;
use virtual_exec_type::mem::{
    Allocator, MemoryAllocator, MemoryAllocatorConstructor, OwnedValue, Value, ValuePtr,
};
use crate::sequential::compile::{compile_offset, GetInstruction};

#[cfg(feature = "unsafe_check")]
#[derive(Clone, Debug)]
pub struct PtrAliveCheck(Arc<RwLock<bool>>);

#[cfg(feature = "unsafe_check")]
impl PtrAliveCheck {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(true)))
    }

    pub fn is_alive(&self) -> bool {
        *self.0.read_arc_safe()
    }

    pub fn clear(&self) -> () {
        *self.0.write_arc_safe() = false;
    }
}

/// The execution instance including the memory allocator and the instruction state machine
#[derive(Debug)]
pub struct Machine<'a> {
    #[allow(unused)]
    /// The memory allocator for the machine
    pub alloc: MemoryAllocator<'a>,
    /// The instruction execution machine for the instance
    pub machine: InstStateMachine<'a>,
    pub resolvers: Vec<MethodResolver>,
    #[cfg(feature = "unsafe_check")]
    pub ptr_alive_check: PtrAliveCheck,
}

#[derive(Debug, Clone)]
pub enum ExprEvalError {
    ExecutionError(ExecutionError),
    ParseError(ParseError)
}

impl Into<ExprEvalError> for ExecutionError {
    fn into(self) -> ExprEvalError {
        ExprEvalError::ExecutionError(self)
    }
}

impl Into<ExprEvalError> for ParseError {
    fn into(self) -> ExprEvalError {
        ExprEvalError::ParseError(self)
    }
}

impl<'a> Machine<'a> {
    /// Create a new execution instance with the given instructions, memory limit and instruction execution limit
    /// # Arguments
    /// * `instructions` - A vector for the sequential instructions
    /// * `memory_lim` - The amount of memory (in virtual bytes) that can be used by the execution instance
    /// * `inst_limit` - The amount of instruction it can run until it being paused by timeout
    ///
    /// # Returns
    /// `Result<Machine, MemoryError>`
    pub fn new(
        instructions: Vec<Instruction>,
        memory_lim: usize,
        inst_limit: u64,
        resolvers: Vec<MethodResolver>,
    ) -> Result<Self, MemoryError> {
        let alloc = MemoryAllocator::construct(memory_lim);
        let mut map = HashMap::new();
        for resolver in resolvers.iter().rev() {
            for item in resolver.get_pair() {
                let ptr = Value::FnPtrExternal(item.0.clone().into_boxed_str(), item.1);
                let alloced = alloc.alloc(ptr)?;
                map.insert(item.0, alloced);
            }
        }
        let machine = InstStateMachine {
            lim: inst_limit,
            fn_stack_frame: vec![FnStackFrame {
                ptr: 0,
                mapping: Arc::new(RwLock::new(map)),
                _acct: None,
            }],
            alloc: alloc.clone(),
            instructions,
            state: Ok(State::Ok),
            stack: vec![],
        };
        Ok(Self {
            alloc,
            machine,
            resolvers,
            #[cfg(feature = "unsafe_check")]
            ptr_alive_check: PtrAliveCheck::new(),
        })
    }

    pub fn get_alloc(&self) -> MemoryAllocator<'a> {
        Arc::clone(&self.alloc)
    }

    #[cfg(feature = "std")]
    fn dispatch_extern_sync(
        &mut self,
        f: &Arc<dyn FnExtern + Send + Sync>,
        values: Vec<ValuePtr<'a>>,
    ) -> Result<ValuePtr<'a>, ExecutionError> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f.fn_extern_sync(self, values)
        })) {
            Ok(result) => result,
            Err(_) => Err(ExecutionError::Critical(
                CriticalError::GenericPanicRewindError,
            )),
        }
    }

    #[cfg(not(feature = "std"))]
    fn dispatch_extern_sync(
        &mut self,
        f: &Arc<dyn FnExtern + Send + Sync>,
        values: Vec<ValuePtr<'a>>,
    ) -> Result<ValuePtr<'a>, ExecutionError> {
        f.fn_extern_sync(self, values)
    }

    pub fn sync_run_once(&mut self) -> Result<(State<'a>, bool), ExecutionError> {
        if let Ok(State::Ok) = self.machine.state {
            self.machine.run_once().map(|x| (x, true))
        } else {
            if let Ok(State::FnExternInput(func, _)) = &self.machine.state {
                let func: Option<Arc<dyn FnExtern + Send + Sync>> =
                    self.resolvers.iter().find_map(|x| x.get(func));
                if let Some(func) = func {
                    let inputs = self.machine.retrieve_fn_input()?.unwrap();
                    let result = self.dispatch_extern_sync(&func, inputs.1);
                    self.machine.push_fn_output(result);
                    return self.machine.state.clone().map(|x| (x, true));
                }
            }
            self.machine.state.clone().map(|x| (x, false))
        }
    }

    pub fn sync_run_for(&mut self, count: u64) -> Result<State<'a>, ExecutionError> {
        for _ in 0..count {
            if let Ok(State::Ok) | Ok(State::FnExternInput(_, _)) = self.machine.state {
                let result = self.sync_run_once()?;
                if !result.1 {
                    return Ok(result.0);
                }
            }
        }
        self.machine.state.clone()
    }

    pub fn sync_run_all(&mut self) -> Result<State<'a>, ExecutionError> {
        while let Ok(State::Ok) | Ok(State::FnExternInput(_, _)) = self.machine.state {
            let result = self.sync_run_once()?;
            if !result.1 {
                return Ok(result.0);
            }
        }
        self.machine.state.clone()
    }

    #[cfg(feature = "async")]
    pub async fn async_run_once(&mut self) -> Result<(State<'a>, bool), ExecutionError> {
        if let Ok(State::Ok) = self.machine.state {
            self.machine.run_once().map(|x| (x, true))
        } else {
            if let Ok(State::FnExternInput(func, _)) = &self.machine.state {
                let func: Option<Arc<dyn FnExtern + Send + Sync>> =
                    self.resolvers.iter().find_map(|x| x.get(func));
                if let Some(func) = func {
                    let inputs = self.machine.retrieve_fn_input()?.unwrap();
                    let result;
                    {
                        let x: &'_ mut Self = &mut *self;
                        result = func.fn_extern_async(x, inputs.1).await;
                    }
                    self.machine.push_fn_output(result);
                    return self.machine.state.clone().map(|x| (x, true));
                }
            }
            self.machine.state.clone().map(|x| (x, false))
        }
    }

    #[cfg(feature = "async")]
    pub async fn async_run_for(&mut self, count: u64) -> Result<State<'a>, ExecutionError> {
        for _ in 0..count {
            if let Ok(State::Ok) | Ok(State::FnExternInput(_, _)) = self.machine.state {
                let result = self.async_run_once().await?;
                if !result.1 {
                    return Ok(result.0);
                }
            }
        }
        self.machine.state.clone()
    }

    #[cfg(feature = "async")]
    pub async fn async_run_all(&mut self) -> Result<State<'a>, ExecutionError> {
        while let Ok(State::Ok) | Ok(State::FnExternInput(_, _)) = self.machine.state {
            let result = self.async_run_once().await?;
            if !result.1 {
                return Ok(result.0);
            }
        }
        self.machine.state.clone()
    }

    pub fn get(&self, name: &str) -> Option<OwnedValue> {
        for fn_frame in self.machine.fn_stack_frame.iter().rev() {
            if let Some(v) = fn_frame.mapping.read_arc_safe().get(name).cloned() {
                return Some(self.alloc.lock_arc_safe().get_owned(&v).unwrap());
            }
        }
        None
    }

    /// Fork the machine to an independent memory space
    pub fn fork<'b>(&self) -> Machine<'b> {
        let forked_inst_state_machine = self.machine.fork();
        let forked_alloc = forked_inst_state_machine.alloc.clone();
        Machine {
            alloc: forked_alloc,
            machine: forked_inst_state_machine,
            resolvers: self.resolvers.clone(),
            #[cfg(feature = "unsafe_check")]
            ptr_alive_check: PtrAliveCheck::new(),
        }
    }

    pub fn push_insts(&mut self, insts: Vec<Instruction>) -> () {
        self.machine.instructions.extend(insts);
        if let Ok(State::Terminated { end_of_instruction: true}) = self.machine.state.clone() {
            self.machine.state = Ok(State::Ok);
        }
    }

    pub fn push_modules(&mut self, module: &Module) -> () {
        let code = compile_offset(&module, self.machine.instructions.len() as u64);
        self.push_insts(code);
    }

    #[cfg(feature = "parse")]
    pub fn push_code(&mut self, code: &str) -> Result<(), virtual_exec_parser::error::ParseError> {
        use virtual_exec_parser::parser::parse;
        let module = parse(code)?;
        self.push_modules(&module);
        Ok(())
    }

    #[cfg(feature = "parse")]
    pub(crate) fn eval_machine<'b>(&self, code: &str) -> Result<Machine<'b>, ParseError> {
        let mut fork = self.fork();
        let expr = parse_expr(&code)?;
        let insts = expr.inst(fork.machine.instructions.len() as u64);
        fork.push_insts(insts);
        Ok(fork)
    }

    /// Note: The following code wouldn't allow runtime controlled execution behaviour(It will execute to the end or failed). Use with caution
    #[cfg(feature = "parse")]
    pub fn eval_sync_all(&self, code: &str) -> Result<OwnedValue, ExprEvalError> {
        let mut machine = self.eval_machine(code).map_err(|x| x.into())?;
        machine.sync_run_all().map_err(|x| x.into())?;
        let ptr = machine.machine.pop_get().map_err(|x| x.into())?;
        Ok(machine.alloc.lock_arc_safe().get_owned(&ptr).unwrap())
    }


    /// Note: The following code wouldn't allow runtime controlled execution behaviour(It will execute to the end or failed). Use with caution
    #[cfg(feature = "parse")]
    pub async fn eval_async_all(&self, code: &str) -> Result<OwnedValue, ExprEvalError> {
        let mut machine = self.eval_machine(code).map_err(|x| x.into())?;
        machine.async_run_all().await.map_err(|x| x.into())?;
        let ptr = machine.machine.pop_get().map_err(|x| x.into())?;
        Ok(machine.alloc.lock_arc_safe().get_owned(&ptr).unwrap())
    }

    fn get_named_module_machine<'b>(&self, module: &Module) -> Machine<'b> {
        let mut alt: Machine<'b> = self.fork();
        let offset = alt.machine.instructions.len();
        // Jmp
        let code = compile_offset(module, offset as u64 + 1);
        // Nop
        let jmp_inst = Instruction::Jmp((offset + 1 + code.iter().len()) as u64); // Jmp to nop
        alt.push_insts(vec![jmp_inst]);
        alt.push_insts(code);
        alt.push_insts(vec![Instruction::Nop]);
        alt.machine.fn_stack_frame.push(FnStackFrame {
            ptr: (offset + 1) as u64,
            mapping: Arc::new(Default::default()),
            _acct: None,
        });
        alt
    }

    pub(crate) fn named_module_post_setup(&mut self, name: &str) -> Result<(), ExecutionError> {
        if let Ok(State::Terminated { end_of_instruction: true}) = self.machine.state.clone() {
            let frame = self.machine.fn_stack_frame.pop().ok_or(ExecutionError::Critical(CriticalError::FnStackUnderflowError))?;
            let items = frame.mapping;
            let first = self.machine.fn_stack_frame.get_mut(0).ok_or(ExecutionError::Critical(CriticalError::FnStackUnderflowError))?;
            first.mapping.write_arc_safe().entry(name.to_string()).insert_entry(self.alloc.alloc(Value::Object(items))?);
            return Ok(())
        }
        Err(ExecutionError::Critical(CriticalError::UnexpectedStateError))
    }

    /// Note: The following code wouldn't allow runtime controlled execution behaviour(It will execute to the end or failed). Use with caution
    /// This should only be used before user code being executed
    pub fn push_named_module_sync_all<'b>(&self, name: &str, module: &Module) -> Result<Machine<'b>, ExecutionError> {
        let mut machine: Machine<'b> = self.get_named_module_machine(module);
        machine.sync_run_all()?;
        machine.named_module_post_setup(name)?;
        Ok(machine)
    }

    /// Note: The following code wouldn't allow runtime controlled execution behaviour(It will execute to the end or failed). Use with caution
    /// This should only be used before user code being executed
    pub async fn push_named_module_async_all<'b>(&self, name: &str, module: &Module) -> Result<Machine<'b>, ExecutionError> {
        let mut machine: Machine<'b> = self.get_named_module_machine(module);
        machine.async_run_all().await?;
        machine.named_module_post_setup(name)?;
        Ok(machine)
    }

    /// Note: The following code wouldn't allow runtime controlled execution behaviour(It will execute to the end or failed). Use with caution
    /// This should only be used before user code being executed
    pub fn load_modules_sync_all<'b>(&self, modules: HashMap<String, Module>) -> Result<Machine<'b>, ExecutionError> {
        let mut machine = self.fork();
        for (name, module) in modules.iter() {
            machine = machine.push_named_module_sync_all(name, module)?;
        };
        Ok(machine)
    }

    /// Note: The following code wouldn't allow runtime controlled execution behaviour(It will execute to the end or failed). Use with caution
    /// This should only be used before user code being executed
    pub async fn load_modules_async_all<'b>(&self, modules: HashMap<String, Module>) -> Result<Machine<'b>, ExecutionError> {
        let mut machine = self.fork();
        for (name, module) in modules.iter() {
            machine = machine.push_named_module_async_all(name, module).await?;
        };
        Ok(machine)
    }

    pub fn push_resolver(&mut self, resolver: MethodResolver) {
        self.resolvers.insert(0, resolver)
    }

    pub fn set_root(&self, key: String, ptr: ValuePtr<'a>) -> Result<(), ExecutionError> {
        self.machine.fn_stack_frame.get(0)
            .ok_or_else(|| ExecutionError::Critical(CriticalError::FnStackUnderflowError))?
            .mapping.write_arc_safe()
            .insert(key, ptr);
        Ok(())
    }

    pub fn set_top(&self, key: String, ptr: ValuePtr<'a>) -> Result<(), ExecutionError> {
        self.machine.fn_stack_frame.last()
            .ok_or_else(|| ExecutionError::Critical(CriticalError::FnStackUnderflowError))?
            .mapping.write_arc_safe()
            .insert(key, ptr);
        Ok(())
    }

    pub fn grant_lim(&mut self, additional: u64) {
        self.machine.grant_lim(additional)
    }

    pub fn reduce_lim(&mut self, size: u64) -> Result<(), RecoverableError> {
        self.machine.reduce_lim(size)
    }

    pub fn check_use(&self, size: u64) -> bool {
        self.machine.check_use(size)
    }
}


impl Drop for Machine<'_> {
    fn drop(&mut self) {
        cfg_if! {
            if #[cfg(feature = "unsafe_check")] {
                self.ptr_alive_check.clear();
            }
        }
    }
}


impl Clone for Machine<'_> {
    fn clone(&self) -> Self {
        Self {
            alloc: self.alloc.clone(),
            machine: self.machine.clone(),
            resolvers: self.resolvers.clone(),
            #[cfg(feature = "unsafe_check")]
            ptr_alive_check: PtrAliveCheck::new(),
        }
    }
}