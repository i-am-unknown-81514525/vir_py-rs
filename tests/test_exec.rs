use virtual_exec::exec;
use virtual_exec_type::exec_ctx::RsValue;

#[test]
fn test_simple_assignment() {
    let code = "a = 1; b = 2; c = 3; if a != b {d = 2;} d;";
    let result = exec(code, 100).unwrap();
    assert_eq!(result.get("a"), Some(&RsValue::Int(1)));
}
