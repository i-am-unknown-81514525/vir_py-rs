use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use async_lock::RwLock;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::fn_extern::{FnExtern, FnExternConstruct};
use virtual_exec_core::Machine;
use virtual_exec_type::base::{TypeCast, VmAnyType};
use virtual_exec_type::error::{CriticalError, ExecutionError, MemoryError, NonRecoverableError};
use virtual_exec_type::ext::SafeWriteArcExt;
use virtual_exec_type::HashMap;
use virtual_exec_type::mem::{Allocator, MemoryAllocator, Value, ValueInnerPtr, ValuePtr};
use crate::core::{lifetime_transmute_machine_ref_mut};
use crate::core::machine_ref::MachineRef;
use crate::types::{extend_ptr, ValuePtrWrapper};

#[wasm_bindgen]
pub struct JsExternFuncSync(Option<js_sys::Function>);

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsValueWrapper(#[wasm_bindgen(getter_with_clone)] pub JsValue);

impl From<JsValue> for JsValueWrapper {
    fn from(v: JsValue) -> Self {
        Self(v)
    }
}


impl VmAnyType for JsValueWrapper {
    fn get_size(&self) -> usize {
        1024
    }
}



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
            clone_construct(&result, &machine.alloc)
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

pub(crate) fn get_id_link(value: &JsValue) -> (usize, HashMap<usize, (JsValue, GraphLink)>) {
    let mut map2: HashMap<usize, (JsValue, GraphLink)> = HashMap::new();
    let mut pending: VecDeque<JsValue> = VecDeque::new();
    let mut seen: Vec<JsValue> = Vec::new();
    pending.push_back(value.clone());
    let id = get_id(&mut seen, &value);
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
            if map2.contains_key(&idx) {
                return;
            }
            pending.push_back(item);
        });
        map2.insert(item_id, (item, link));
    }
    (id, map2)
}
fn to_value_uninit<'a>(js: &JsValue) -> Value<'a> {
    if let Some(v) = js.as_f64() {
        Value::Float(v)
    } else if let Some(v) = js.as_string() {
        Value::String(v.into_boxed_str())
    } else if let Some(v) = js.as_bool() {
        Value::Bool(v)
    } else if js.is_null_or_undefined() {
        Value::None
    } else if js.is_array() {
        Value::Collection(Arc::new(RwLock::new(Vec::new())))
    } else if js.is_object() {
        Value::Object(Arc::new(RwLock::new(HashMap::new())))
    } else {
        Value::Any(Arc::new(RwLock::new(JsValueWrapper::from(js.clone()))))
    }
}

fn clone_construct<'a>(value: &JsValue, alloc: &MemoryAllocator<'a>) -> Result<ValuePtr<'a>, ExecutionError> {
    let (id, link_map) = get_id_link(&value);
    let mut new_ref: HashMap<usize, (ValuePtr<'a>, GraphLink)> = link_map.into_iter()
        .map(|(k, v)| Ok((k, (alloc.alloc(to_value_uninit(&v.0))?, v.1))))
        .collect::<Result<HashMap<usize, (ValuePtr<'a>, GraphLink)>, MemoryError>>()?;
    let immutable: HashMap<usize, ValuePtr<'a>> = new_ref.iter().map(|(k, v)| (k.clone(), v.0.clone())).collect();
    for (k, (v, link)) in new_ref.iter_mut() {
        match link {
            GraphLink::Object(map) => {
                if let Some(inner) = v.as_object() {
                    let mut lock = inner.write_arc_safe();
                    for (k, v) in map.iter() {
                        let ptr = immutable.get(v).expect("Value in new_ref should exist in immutable");
                        lock.insert(k.clone(), ptr.clone());
                    }
                } else {
                    unreachable!("Should be object on creation")
                }
            },
            GraphLink::Collection(vec) => {
                if let Some(inner) = v.as_collections() {
                    let mut lock = inner.write_arc_safe();
                    for v in vec.iter() {
                        let ptr = immutable.get(v).expect("Value in new_ref should exist in immutable");
                        lock.push(ptr.clone());
                    }
                } else {
                    unreachable!("Should be collection on creation")
                }
            },
            GraphLink::None => {}
        }
    };
    Ok(new_ref.remove(&id).unwrap().0)
}