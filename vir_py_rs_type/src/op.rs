use crate::base::Value;
use bumpalo::Bump;

type OpFn =
    for<'ctx> fn(lhs: Value<'ctx>, rhs: Value<'ctx>, arena: &'ctx Bump) -> Option<Value<'ctx>>;

#[macro_export]
macro_rules! __op_register {
    (
        $lhs_type:ty,
        $rhs_type:ty,
        $func:expr,
        $output_wrapper:path,
        $impl_path:path
    ) => {
        const _: () = {
            fn _op_impl<'ctx>(
                lhs: $crate::base::Value<'ctx>,
                rhs: $crate::base::Value<'ctx>,
                arena: &'ctx ::bumpalo::Bump,
            ) -> Option<$crate::base::Value<'ctx>> {
                let lhs_val = <$lhs_type as $crate::base::Downcast>::from_value(lhs)?;
                let rhs_val = <$rhs_type as $crate::base::Downcast>::from_value(rhs)?;
                match $func(lhs_val.clone(), rhs_val.clone()) {
                    Ok(result) => {
                        Some($crate::base::ValueContainer::new(
                            $output_wrapper(result),
                            arena,
                        ))
                    }
                    Err(err) => {
                        Some($crate::base::ValueContainer::new(
                            $crate::base::ValueKind::ErrorWrapped(err),
                            arena,
                        ))
                    }
                }
            }

            ::inventory::submit! {
                $impl_path { function: _op_impl }
            };
        };
    };
}

macro_rules! __op_create {
    ($name:tt, $alt_name:tt, $op:tt) => {
        __op_create!(@impl $name, @impl $alt_name, $op, $);
    };
    (@impl $name:tt, @impl $alt_name:tt, $op:tt, $d:tt) => {
        ::paste::paste!{
            pub struct [< Op $alt_name Impl>] {pub function: OpFn }
            ::inventory::collect!([< Op $alt_name Impl>]);
            pub fn [< op_ $name>]<'ctx>(lhs: $crate::base::Value<'ctx>, rhs: $crate::base::Value<'ctx>, arena: &'ctx ::bumpalo::Bump) -> ::core::option::Option<$crate::base::Value<'ctx>> {
                for implementation in ::inventory::iter::<[<Op $alt_name Impl>]> {
                    if let ::core::option::Option::Some(result) = (implementation.function)(lhs, rhs, arena) {
                        return Some(result);
                    }
                }
                None
            }
            #[macro_export]
            macro_rules! [<register_op_ $name>] {
                ($d lhs_type:ty, $d rhs_type:ty, $d output_wrapper:path) => {
                    [<register_op_ $name>]!($d lhs_type, $d rhs_type, $d output_wrapper, |a, b| a $op b);
                };
                ($d lhs_type:ty, $d rhs_type:ty, $d output_wrapper:path, $d func:expr) => {
                    $crate::__op_register!($d lhs_type, $d rhs_type, $d func, $d output_wrapper, $crate::op::[<Op $alt_name Impl>]);
                }
            }
        }
    };
}

__op_create!(add, Add, +);
__op_create!(sub, Sub, -);
__op_create!(mul, Mul, *);
__op_create!(div, Div, /);
__op_create!(eq, Eq, ==);
__op_create!(ge, Ge, >=);
__op_create!(gt, Gt, >);
__op_create!(le, Le, <=);
__op_create!(lt, Lt, <);
__op_create!(ne, Ne, !=);
__op_create!(moduls, Mod, %);
__op_create!(bsl, Bsl, <<);
__op_create!(bsr, Bsr, >>);
__op_create!(band, BitwiseAnd, &);
__op_create!(bor, BitwiseOr, |);
__op_create!(bxor, BitwiseXor, ^);
__op_create!(not, Not, !);