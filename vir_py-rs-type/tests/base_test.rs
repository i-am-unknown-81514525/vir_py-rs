use bumpalo::Bump;
use vir_py_rs_type::base::{ValueContainer, VirPyType, VirPyTypeMut};
use vir_py_rs_type::builtin::VirPyInt;


#[test]
fn test_value_container() {
    let arena = Bump::new();
    let mut container = ValueContainer::new(VirPyInt::new(42), &arena);
    container.downcast_mut::<VirPyInt>().unwrap().set_value(100);
    println!("{:?}", container);
}
