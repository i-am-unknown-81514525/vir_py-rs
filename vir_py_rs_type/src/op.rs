use crate::base::{ValueContainer, VirPyType, VirPyTypeMut};
use crate::builtin::{VirPyFloat, VirPyInt};
use bumpalo::Bump;


type OpAddFn = for<'ctx> fn(
    &ValueContainer<'ctx>,
    &ValueContainer<'ctx>,
    &'ctx Bump,
) -> Option<ValueContainer<'ctx>>;


pub struct OpAddImpl {
    pub function: OpAddFn,
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
macro_rules! register_op_add {
    ($lhs_type:ty, $rhs_type:ty, $out_type:ty) => {
        const _: () = {
            fn op_add_impl<'ctx>(
                lhs: &$crate::base::ValueContainer<'ctx>,
                rhs: &$crate::base::ValueContainer<'ctx>,
                arena: &'ctx ::bumpalo::Bump,
            ) -> ::core::option::Option<$crate::base::ValueContainer<'ctx>> {
                let lhs_val = lhs.downcast_ref::<$lhs_type>()?;
                let rhs_val = rhs.downcast_ref::<$rhs_type>()?;
                let result = *lhs_val + *rhs_val;
                ::core::option::Option::Some($crate::base::ValueContainer::new(result, arena))
            }

            ::inventory::submit! {
                $crate::op::OpAddImpl { function: op_add_impl }
            }
        };
    };

    ($lhs_type:ty, $rhs_type:ty, $out_type:ty, $func:expr) => {
        const _: () = {
            fn op_add_impl<'ctx>(
                lhs: &$crate::base::ValueContainer<'ctx>,
                rhs: &$crate::base::ValueContainer<'ctx>,
                arena: &'ctx ::bumpalo::Bump,
            ) -> ::core::option::Option<$crate::base::ValueContainer<'ctx>> {
                let lhs_val: $lhs_type = lhs.downcast_ref::<$lhs_type>()?;
                let rhs_val: $rhs_type = rhs.downcast_ref::<$rhs_type>()?;
                let result: $out_type = $func(lhs_val, rhs_val);
                ::core::option::Option::Some(ValueContainer::new(result, arena))
            }

            ::inventory::submit! {
                $crate::op::OpAddImpl { function: op_add_impl }
            }
        }
    }
}


register_op_add!(VirPyInt, VirPyInt, VirPyInt);
register_op_add!(VirPyFloat, VirPyFloat, VirPyFloat);
register_op_add!(VirPyFloat, VirPyInt, VirPyFloat);
register_op_add!(VirPyInt, VirPyFloat, VirPyFloat);