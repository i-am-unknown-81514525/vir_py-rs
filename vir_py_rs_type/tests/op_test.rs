use bumpalo::Bump;
use vir_py_rs_type::base::{ValueContainer, ValueKind};
use vir_py_rs_type::builtin::{VirPyFloat, VirPyInt};
use vir_py_rs_type::op::op_add;

#[test]
fn test_op_add_functionality() {
    let arena = Bump::new();

    let lhs_int = ValueContainer::new(ValueKind::Int(VirPyInt::new(15)), &arena);
    let rhs_int = ValueContainer::new(ValueKind::Int(VirPyInt::new(27)), &arena);

    let lhs_float = ValueContainer::new(ValueKind::Float(VirPyFloat::new(1.5)), &arena);
    let rhs_float = ValueContainer::new(ValueKind::Float(VirPyFloat::new(2.25)), &arena);

    let result_int = op_add(lhs_int, rhs_int, &arena).expect("op_add for Int+Int should succeed");
    assert_eq!(result_int.as_int().unwrap().value, 42);
    println!("Int + Int works as expected.");

    let result_float =
        op_add(lhs_float, rhs_float, &arena).expect("op_add for Float+Float should succeed");
    // Use a small epsilon for float comparison
    assert!((result_float.as_float().unwrap().value - 3.75).abs() < f64::EPSILON);
    println!("Float + Float works as expected.");

    let result_unsupported =
        op_add(lhs_int, lhs_float, &arena).expect("op_add for Int+Float should succeed");
    assert!(result_unsupported.as_float().unwrap().value - 16.5 < f64::EPSILON);
    println!("Int + Float works as expected.");
}
