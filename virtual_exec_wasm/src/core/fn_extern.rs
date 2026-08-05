use std::collections::{HashSet, VecDeque};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::fn_extern::{FnExtern, FnExternConstruct};
use virtual_exec_core::Machine;
use virtual_exec_type::error::{CriticalError, ExecutionError, NonRecoverableError};
use virtual_exec_type::HashMap;
use virtual_exec_type::mem::{ValueInnerPtr, ValuePtr};
use crate::core::{lifetime_transmute_machine_ref_mut};
use crate::core::machine_ref::MachineRef;
use crate::types::{extend_ptr, ValuePtrWrapper};

#[wasm_bindgen]
pub struct JsExternFuncSync(Option<js_sys::Function>);

impl FnExternConstruct for JsExternFuncSync {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self(None)
    }
}

impl From<js_sys::Function> for JsExternFuncSync {
    fn from(value: js_sys::Function) -> Self {
        Self(Some(value))
    }
}

impl FnExtern for JsExternFuncSync {
    fn fn_extern_sync<'a, 'b>(&self, machine: &'b mut Machine<'a>, values: Vec<ValuePtr<'a>>) -> Result<ValuePtr<'a>, ExecutionError> {
        if let Some(func) = &self.0 {
            let mut machine_ref = MachineRef::from(lifetime_transmute_machine_ref_mut(machine));
            let values_conv: Vec<ValuePtrWrapper> = values.into_iter().map(extend_ptr).map(ValuePtrWrapper::from).collect();
            let result = unsafe {
                machine_ref.with_guarded(|m| {
                    let a = JsValue::from(m);
                    let b = JsValue::from(values_conv);
                    func.call2(&JsValue::NULL, &a, &b)
                })
            };
            let result = match result {
                Some(Ok(v)) => Ok(v),
                Some(Err(e)) => Err(ExecutionError::NonRecoverable(NonRecoverableError::GenericError)),
                None => Err(ExecutionError::Critical(CriticalError::UnexpectedStateError))
            }?;
            todo!()
        } else {
            Err(ExecutionError::NonRecoverable(NonRecoverableError::UnexpectedFunctionCall))
        }
    }

    fn get_size(&self) -> usize {
        1024
    }
}


pub enum GraphLink {
    Object(HashMap<String, usize>),
    Collection(Vec<usize>),
    None
}

fn is_ptr_eq(left: &JsValue, right: &JsValue) -> bool {
    left == right || (left.as_f64().is_some_and(f64::is_nan) && right.as_f64().is_some_and(f64::is_nan))
}

fn get_id(seen: &mut Vec<JsValue>, v: &JsValue) -> usize {
    if let Some(i) = seen.iter().position(|s| {
        is_ptr_eq(s, v)
    }) {
        return i;
    }
    seen.push(v.clone());
    seen.len() - 1
}

pub(crate) fn get_id_link(value: &JsValue) -> (HashMap<usize, GraphLink>, HashMap<usize, JsValue>) {
    let mut map1: HashMap<usize, GraphLink> = HashMap::new();
    let mut map2: HashMap<usize, JsValue> = HashMap::new();
    let mut pending: VecDeque<JsValue> = VecDeque::new();
    let mut seen: Vec<JsValue> = Vec::new();
    pending.push_back(value.clone());
    while let Some(item) = pending.pop_front() {
        let item_id = get_id(&mut seen, &item);
        let mut discovered: Vec<(JsValue, usize)> = Vec::new();
        let link = if item.is_array() {
            let arr = js_sys::Array::from(&item);
            GraphLink::Collection(arr.iter().map(|item| {
                let id = get_id(&mut seen, &item);
                discovered.push((item, id));
                id
            }).collect())
        } else if item.is_object() && !item.is_null() && !item.is_undefined() {
            let obj = js_sys::Object::from(item.clone());
            let entries = js_sys::Object::entries(&obj);
            GraphLink::Object(entries.iter().map(|entry| {
                let pair = js_sys::Array::try_from(entry).unwrap();
                let id = get_id(&mut seen, &pair.get(1));
                discovered.push((pair.get(1), id));
                (pair.get(0).as_string().unwrap_or_default(), id)
            }).collect())
        }  else {
            GraphLink::None
        };
        discovered.into_iter().for_each(|(item, idx)| {
            if pending.contains(&item) {
                return;
            }
            if map1.contains_key(&idx) {
                return;
            }
            pending.push_back(item);
        });
        map2.insert(item_id, item);
        map1.insert(item_id, link);
    }
    (map1, map2)
}
