use bumpalo::Bump;
use vir_py_rs_type::base::ValueContainer;
use vir_py_rs_type::builtin::{VirPyFloat, VirPyInt};
use vir_py_rs_type::op::op_add;

#[test]
fn test_op_add_auto_registration() {
    let arena = Bump::new();

    // Test int + int
    let lhs_int = ValueContainer::new(VirPyInt::new(15), &arena);
    let rhs_int = ValueContainer::new(VirPyInt::new(27), &arena);
    let result_int_container = op_add(&lhs_int, &rhs_int, &arena).unwrap();
    let result_int = result_int_container.downcast_ref::<VirPyInt>().unwrap();
    assert_eq!(result_int.get_value(), 42);
    println!("Int + Int result: {:?}", result_int);


    // Test float + float
    let lhs_float = ValueContainer::new(VirPyFloat::new(1.5), &arena);
    let rhs_float = ValueContainer::new(VirPyFloat::new(2.25), &arena);
    let result_float_container = op_add(&lhs_float, &rhs_float, &arena).unwrap();
    let result_float = result_float_container
        .downcast_ref::<VirPyFloat>()
        .unwrap();
    assert_eq!(result_float.get_value(), 3.75);
    println!("Float + Float result: {:?}", result_float);


    // Test unregistered combination (int + float)
    let result_unsupported = op_add(&lhs_int, &rhs_float, &arena);
    assert!(result_unsupported.is_none());
    println!("Int + Float result: None (as expected)");
}
