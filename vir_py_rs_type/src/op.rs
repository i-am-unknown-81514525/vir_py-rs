use crate::base::{ValueContainer};
use bumpalo::Bump;


type OpFn = for<'ctx> fn(
    &ValueContainer<'ctx>,
    &ValueContainer<'ctx>,
    &'ctx Bump,
) -> Option<ValueContainer<'ctx>>;

#[macro_export]
macro_rules! __op_register {
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
                ::core::option::Option::Some($crate::base::ValueContainer::new(result, arena))
            }

            ::inventory::submit! {
                $impl_path { function: _op_impl }
            }
        };
    };
}

#[macro_export]
macro_rules! __op_create {
    ($name:tt, $trait_ty:ident, $op:tt) => {
        $crate::__op_create!(@impl $name, $trait_ty, $op, $);
    };
    (@impl $name:tt, $trait_ty:ident, $op:tt, $d:tt) => {
        ::paste::paste!{
            pub struct [< Op $trait_ty Impl >]  {
                pub function: $crate::op::OpFn,
            }
            ::inventory::collect!([< Op $trait_ty Impl >]);

            pub fn [< op_ $name >] <'ctx>(
                lhs: &$crate::base::ValueContainer<'ctx>,
                rhs: &$crate::base::ValueContainer<'ctx>,
                arena: &'ctx ::bumpalo::Bump,
            ) -> Option<$crate::base::ValueContainer<'ctx>> {
                for implementation in ::inventory::iter::<[< Op $trait_ty Impl >]> {
                    if let ::core::option::Option::Some(result) = (implementation.function)(lhs, rhs, arena) {
                        return ::core::option::Option::Some(result);
                    }
                }
                ::core::option::Option::None
            }

            #[macro_export]
            macro_rules! [< register_op_ $name >] {
                ($d lhs_type:ty, $d rhs_type:ty, $d out_type:ty) => {
                    const _: () = {
                        fn _op<T>(lhs: &T, rhs: &$d rhs_type) -> $d out_type where T: ::std::ops::$trait_ty<$d rhs_type, Output=$d out_type> + Clone {
                            lhs.clone() $op *rhs
                        }
                        [< register_op_ $name >]!($d lhs_type, $d rhs_type, $d out_type, _op);
                    };
                };

                ($d lhs_type:ty, $d rhs_type:ty, $d out_type:ty, $d func:expr) => {
                    $crate::__op_register!($d lhs_type, $d rhs_type, $d out_type, $d func, $crate::op::[< Op $trait_ty Impl >]);
                }
            }
        }
    };
}


__op_create!(add, Add, +);
__op_create!(sub, Sub, -);
__op_create!(mul, Mul, *);
__op_create!(div, Div, /);