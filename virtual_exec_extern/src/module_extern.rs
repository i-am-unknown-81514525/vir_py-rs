pub use cps::{include, cps};
use virtual_exec_macro::{compile, parse};
pub use virtual_exec_macro::relative;



#[macro_export]
#[cps]
macro_rules! import_parse {
    ($file_path:expr) =>
     let $($file_content:tt)* = $crate::module_extern::include!($file_path) in
        {
            parse!{ $($file_content)* }
        }
}


#[macro_export]
macro_rules! import_compile {
    ($file_path:expr) => {
        compile!(import_parse!($file_path))
    };
}

pub use import_compile;
