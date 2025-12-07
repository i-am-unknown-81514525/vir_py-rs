use crate::base::ValueKind;
use crate::builtin::*;

register_op_add!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_add!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_add!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_add!(VirPyFloat, VirPyInt, ValueKind::Float);

register_op_sub!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_sub!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_sub!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_sub!(VirPyFloat, VirPyInt, ValueKind::Float);

register_op_mul!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_mul!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_mul!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_mul!(VirPyFloat, VirPyInt, ValueKind::Float);

register_op_div!(VirPyInt, VirPyInt, ValueKind::Float);
register_op_div!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_div!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_div!(VirPyFloat, VirPyInt, ValueKind::Float);

register_op_moduls!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_moduls!(VirPyFloat, VirPyFloat, ValueKind::Float);
register_op_moduls!(VirPyInt, VirPyFloat, ValueKind::Float);
register_op_moduls!(VirPyFloat, VirPyInt, ValueKind::Float);

register_op_eq!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a == b));
register_op_le!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a <= b));
register_op_lt!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(!a & b));
register_op_ge!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a >= b));
register_op_gt!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a & !b));
register_op_ne!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a != b));

register_op_eq!(
    VirPyInt,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyInt| Ok(a.value == b.value)
);
register_op_eq!(
    VirPyInt,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 == b.value)
);
register_op_eq!(
    VirPyFloat,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyInt| Ok(a.value == b.value as f64)
);
register_op_eq!(
    VirPyFloat,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyFloat| Ok(a.value == b.value)
);

register_op_le!(
    VirPyInt,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyInt| Ok(a.value <= b.value)
);
register_op_le!(
    VirPyInt,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 <= b.value)
);
register_op_le!(
    VirPyFloat,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyInt| Ok(a.value <= b.value as f64)
);
register_op_le!(
    VirPyFloat,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyFloat| Ok(a.value <= b.value)
);

register_op_ge!(
    VirPyInt,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyInt| Ok(a.value >= b.value)
);
register_op_ge!(
    VirPyInt,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 >= b.value)
);
register_op_ge!(
    VirPyFloat,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyInt| Ok(a.value >= b.value as f64)
);
register_op_ge!(
    VirPyFloat,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyFloat| Ok(a.value >= b.value)
);

register_op_lt!(
    VirPyInt,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyInt| Ok(a.value < b.value)
);
register_op_lt!(
    VirPyInt,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyFloat| Ok((a.value as f64) < b.value)
);
register_op_lt!(
    VirPyFloat,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyInt| Ok(a.value < b.value as f64)
);
register_op_lt!(
    VirPyFloat,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyFloat| Ok(a.value < b.value)
);

register_op_gt!(
    VirPyInt,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyInt| Ok(a.value > b.value)
);
register_op_gt!(
    VirPyInt,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 > b.value)
);
register_op_gt!(
    VirPyFloat,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyInt| Ok(a.value > b.value as f64)
);
register_op_gt!(
    VirPyFloat,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyFloat| Ok(a.value > b.value)
);

register_op_ne!(
    VirPyInt,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyInt| Ok(a.value != b.value)
);
register_op_ne!(
    VirPyInt,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyInt, b: VirPyFloat| Ok(a.value as f64 != b.value)
);
register_op_ne!(
    VirPyFloat,
    VirPyInt,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyInt| Ok(a.value != b.value as f64)
);
register_op_ne!(
    VirPyFloat,
    VirPyFloat,
    ValueKind::Bool,
    |a: VirPyFloat, b: VirPyFloat| Ok(a.value != b.value)
);

register_op_bsl!(
    VirPyInt,
    VirPyInt,
    ValueKind::Int,
    |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value << b.value))
);
register_op_bsr!(
    VirPyInt,
    VirPyInt,
    ValueKind::Int,
    |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value >> b.value))
);
register_op_band!(
    VirPyInt,
    VirPyInt,
    ValueKind::Int,
    |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value & b.value))
);
register_op_bor!(
    VirPyInt,
    VirPyInt,
    ValueKind::Int,
    |a: VirPyInt, b: VirPyInt| Ok(VirPyInt::new(a.value | b.value))
);
register_op_and!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a && b));
register_op_or!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a || b));
register_op_bxor!(bool, bool, ValueKind::Bool, |a: bool, b: bool| Ok(a ^ b));
// register_op_bsl!(VirPyFloat, VirPyInt, ValueKind::Float, |a: VirPyFloat, b: VirPyInt| Ok(VirPyFloat::new(a.value << b.value)));
// register_op_bsr!(VirPyFloat, VirPyInt, ValueKind::Float, |a: VirPyFloat, b: VirPyInt| Ok(VirPyFloat::new(a.value >> b.value)));
// Best attempt would be using i64::pow, but overflow would be a big issue

register_op_not!(bool, ValueKind::Bool, |a: bool| Ok(!a));
register_op_pos!(VirPyInt, ValueKind::Int, |a: VirPyInt| Ok(VirPyInt::new(
    a.value
)));
register_op_pos!(VirPyFloat, ValueKind::Float, |a: VirPyFloat| Ok(
    VirPyFloat::new(a.value)
));
register_op_neg!(VirPyInt, ValueKind::Int, |a: VirPyInt| Ok(VirPyInt::new(
    -a.value
)));
register_op_neg!(VirPyFloat, ValueKind::Float, |a: VirPyFloat| Ok(
    VirPyFloat::new(-a.value)
));

register_op_add!(
    String,
    String,
    ValueKind::String,
    |a: String, b: String| Ok(format!("{}{}", a, b).to_string())
);
