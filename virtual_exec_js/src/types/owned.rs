use std::ops::Deref;
use std::sync::Arc;
use wasm_bindgen::{JsCast, JsValue};
use virtual_exec_type::mem::{get_all_owned_value, OwnedValue, OwnedValueInternal, ValuePtr};
use crate::error::Error;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_type::error::MemoryError;
use virtual_exec_type::ext::SafeReadArcExt;
use virtual_exec_type::HashMap;
use crate::Dewrap;

#[wasm_bindgen]
pub struct OwnedValueWrapper {
    inner: OwnedValue
}

impl OwnedValueWrapper {
    
    pub fn js_conv<T: Error>(res: Result<OwnedValue, T>, prefix: &str) -> Result<Self, JsValue> {
        match res {
            Ok(v) => Ok(Self::from(v)),
            Err(e) => Err(e.to_js_error(prefix))
        }
    }
}

impl From<OwnedValue> for OwnedValueWrapper {
    fn from(value: OwnedValue) -> Self {
        Self {
            inner: value
        }
    }
}

impl Dewrap<OwnedValue> for OwnedValueWrapper {
    fn dewrap(&self) -> OwnedValue {
        self.inner.clone()
    }
}

#[wasm_bindgen]
pub struct JsDptr {
    pub ptr: u64,
    pub size: usize
}

#[wasm_bindgen]
pub struct JsExternPtr {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    pub size: usize
}

impl From<(u64, usize)> for JsDptr {
    fn from(value: (u64, usize)) -> Self {
        Self{ptr: value.0, size: value.1}
    }
}

impl From<(String, usize)> for JsExternPtr {
    fn from(value: (String, usize)) -> Self {
        Self { name: value.0, size: value.1 }
    }
}

fn to_js_value_uninit(value: &OwnedValue) -> JsValue {
    match value.read_arc_safe().deref() {
        OwnedValueInternal::Int(x) => (*x as f64).into(),
        OwnedValueInternal::Float(x) => (*x).into(),
        OwnedValueInternal::Bool(x) => (*x).into(),
        OwnedValueInternal::String(x) => (*x).to_string().into(),
        OwnedValueInternal::None => JsValue::NULL,
        OwnedValueInternal::Error(e) => e.to_js_error("Unknown"),
        OwnedValueInternal::DPtr(loc, arg_len) =>
            JsDptr::from((*loc, *arg_len)).into(),
        OwnedValueInternal::FnPtrExternal(name, arg_len) =>
            JsExternPtr::from((name.to_string(), *arg_len)).into(),
        OwnedValueInternal::Collection(_) => js_sys::Array::new().into(),
        OwnedValueInternal::Object(_) => js_sys::Object::new().into(),
    }
}

#[wasm_bindgen]
impl OwnedValueWrapper {
    #[wasm_bindgen]
    pub fn to_js(&self) -> JsValue {
        let all = get_all_owned_value(self.inner.clone());
        let mixed: Vec<(OwnedValue, JsValue)> = all
            .iter()
            .map(
                |x| {
                    let ptr = to_js_value_uninit(x);
                    (x.clone(), ptr)
                }
            ).collect();
        let mut immutable: Vec<JsValue> = mixed.iter()
            .map(|x| x.1.clone()).collect::<Vec<_>>();
        for (owned, ptr) in mixed.iter() {
            let v = owned.read_arc_safe();
            let get_idx = |x|
                all.iter().position(|y| Arc::ptr_eq(x, y));
            match v.clone() {
                OwnedValueInternal::Collection(c) => {
                    let idx = c.iter().map(get_idx).collect::<Option<Vec<_>>>().expect("All element should exist in all");
                    let vec = ptr.unchecked_ref::<js_sys::Array>();
                    idx.into_iter().for_each(|i| {
                            vec.push(
                                &immutable.get(i)
                                    .expect("Index should exist in immutable as it is collected from get_idx")
                                    .clone()
                            );
                        }
                    );
                },
                OwnedValueInternal::Object(o) => {
                    let idx_map = o.iter()
                        .map(|(k, v)| get_idx(v).map(|v| (k.clone(), v)))
                        .collect::<Option<HashMap<String, usize>>>().expect("All element should exist in all");
                    let map = ptr.unchecked_ref::<js_sys::Object>();
                    idx_map.into_iter().for_each(|(k, i)|
                        {
                            let ptr = immutable.get(i).expect("Index should exist in immutable as it is collected from get_idx").clone();
                            js_sys::Reflect::set(&map, &JsValue::from(k), &JsValue::from(ptr))
                                .expect("Set on map shouldn't fail");
                            ()
                        }
                    );
                },
                _ => {}
            }
        }
        immutable.swap_remove(0)
    }
}