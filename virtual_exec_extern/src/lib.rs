pub mod fn_extern;
pub use virtual_exec_macro::{fn_extern_wrap, fn_extern_wrap_async};

extern crate alloc;

pub mod __private {
    pub use alloc::string::String;
    pub use alloc::sync::Arc;
}
