use bumpalo::Bump;
use vir_py_rs_type::base::{ValueContainer, ValueKind};
use vir_py_rs_type::builtin::VirPyInt;

#[test]
fn test_value_creation_and_downcast() {
    let arena = Bump::new();
    let int_kind = ValueKind::Int(VirPyInt::new(42));
    let value_handle = ValueContainer::new(int_kind, &arena);
    let extracted_int = value_handle.as_int().expect("Downcast to Int failed");
    assert_eq!(extracted_int.value, 42);
    println!(
        "Successfully created and downcasted value: {:?}",
        value_handle
    );
}
