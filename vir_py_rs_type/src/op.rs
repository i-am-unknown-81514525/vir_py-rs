use crate::base::{ValueContainer};
use crate::builtin::{VirPyFloat, VirPyInt};
use bumpalo::Bump;


type OpFn = for<'ctx> fn(
    &ValueContainer<'ctx>,
    &ValueContainer<'ctx>,
    &'ctx Bump,
) -> Option<ValueContainer<'ctx>>;


pub struct OpAddImpl {
    pub function: OpFn,
}

inventory::collect!(OpAddImpl);

pub fn op_add<'ctx>(
    lhs: &ValueContainer<'ctx>,
    rhs: &ValueContainer<'ctx>,
    arena: &'ctx Bump,
) -> Option<ValueContainer<'ctx>> {
    for implementation in inventory::iter::<OpAddImpl> {
        if let Some(result) = (implementation.function)(lhs, rhs, arena) {
            return Some(result);
        }
    }
    None
}

#[macro_export]
macro_rules! op_register {
    ($lhs_type:ty, $rhs_type:ty, $out_type:ty, $func:expr, $impl_path:path) => {
        const _: () = {
            fn _op_impl<'ctx>(
                lhs: &$crate::base::ValueContainer<'ctx>,
                rhs: &$crate::base::ValueContainer<'ctx>,
                arena: &'ctx ::bumpalo::Bump,
            ) -> ::core::option::Option<$crate::base::ValueContainer<'ctx>> {
                let lhs_val = lhs.downcast_ref::<$lhs_type>()?;
                let rhs_val= rhs.downcast_ref::<$rhs_type>()?;
                let result: $out_type = $func(lhs_val, rhs_val);
                ::core::option::Option::Some(ValueContainer::new(result, arena))
            }

            ::inventory::submit! {
                $impl_path { function: _op_impl }
            }
        };
    };
}

#[macro_export]
macro_rules! register_op_add {
    ($lhs_type:ty, $rhs_type:ty, $out_type:ty) => {
        const _: () = {
            fn _op<T>(lhs: &T, rhs: &$rhs_type) -> $out_type where T: ::std::ops::Add<$rhs_type, Output=$out_type> + Clone {
                lhs.clone() + *rhs
            }
            $crate::register_op_add!($lhs_type, $rhs_type, $out_type, _op);
        };
    };

    ($lhs_type:ty, $rhs_type:ty, $out_type:ty, $func:expr) => {
        $crate::op_register!($lhs_type, $rhs_type, $out_type, $func, $crate::op::OpAddImpl)
    }
}


register_op_add!(VirPyInt, VirPyInt, VirPyInt);
register_op_add!(VirPyFloat, VirPyFloat, VirPyFloat);
register_op_add!(VirPyFloat, VirPyInt, VirPyFloat);
register_op_add!(VirPyInt, VirPyFloat, VirPyFloat);


pub struct OpSubImpl {
    pub function: OpFn,
}

inventory::collect!(OpSubImpl);

pub fn op_sub<'ctx>(
    lhs: &ValueContainer<'ctx>,
    rhs: &ValueContainer<'ctx>,
    arena: &'ctx Bump,
) -> Option<ValueContainer<'ctx>> {
    for implementation in inventory::iter::<OpSubImpl> {
        if let Some(result) = (implementation.function)(lhs, rhs, arena) {
            return Some(result);
        }
    }
    None
}

#[macro_export]
macro_rules! register_op_sub {
    ($lhs_type:ty, $rhs_type:ty, $out_type:ty) => {
        const _: () = {
            fn _op<T>(lhs: &T, rhs: &$rhs_type) -> $out_type where T: ::std::ops::Sub<$rhs_type, Output=$out_type> + Clone {
                lhs.clone() - *rhs
            }
            $crate::register_op_add!($lhs_type, $rhs_type, $out_type, _op);
        };
    };

    ($lhs_type:ty, $rhs_type:ty, $out_type:ty, $func:expr) => {
        $crate::op_register!($lhs_type, $rhs_type, $out_type, $func, $crate::op::OpSubImpl)
    }
}


register_op_sub!(VirPyInt, VirPyInt, VirPyInt);
register_op_sub!(VirPyFloat, VirPyFloat, VirPyFloat);
register_op_sub!(VirPyFloat, VirPyInt, VirPyFloat);
register_op_sub!(VirPyInt, VirPyFloat, VirPyFloat);