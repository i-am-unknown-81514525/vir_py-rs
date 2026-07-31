pub use virtual_exec_macro::{parse, import_parse_relative};


/// Produce a Hashmap<String, Module> for the loaded module
#[macro_export]
macro_rules! load_vel_module {
    (@load_internal std) => {
        $crate::native::import_parse_relative!("src/native_std/std.vel")
    };
    (@load_internal $($token:tt)*) => {
        $crate::native::parse!($($token)*)
    };

    (@resolve $map:expr, $name:ident $expression:expr) => {
        $map.insert(stringify!($name).to_string(), $expression);
    };
    (@parse $map:expr $(,)?) => {};
    (@parse $map:expr, load $name:ident  $(;)?) => {
        $crate::load_vel_module!(@resolve $map, $name $crate::load_vel_module!(@load_internal $name));
    };
    (@parse $map:expr, load $name:ident; $($later:tt)*) => {
        $crate::load_vel_module!(@resolve $map, $name $crate::load_vel_module!(@load_internal $name));
        $crate::load_vel_module!(@parse $map, $($later)*);
    };
    (@parse $map:expr, $name:ident => { $($token:tt)* } $(;)?) => {
        $crate::load_vel_module!(@resolve $map, $name $crate::load_vel_module!(@load_internal $($token)*));
    };
    (@parse $map:expr, $name:ident => { $($token:tt)* } $(;)? $later_name:ident $($later:tt)*) => {
        $crate::load_vel_module!(@resolve $map, $name $crate::load_vel_module!(@load_internal $($token)*));
        $crate::load_vel_module!(@parse $map, $later_name $($later)*);
    };
    ($($token:tt)*) => {
        {
            let mut map: ::std::collections::HashMap<::std::string::String, ::virtual_exec_type::ast::core::Module> = ::std::collections::HashMap::new();
            $crate::load_vel_module!(@parse map, $($token)*);
            map
        }
    }
}

fn _a() {
    load_vel_module!(
        load std;
        test => {
            fn a() {
                return None;
            }
        }
    );
}