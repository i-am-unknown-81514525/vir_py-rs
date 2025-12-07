use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::rc::Rc;
use bumpalo::Bump;
use vir_py_rs_macro::parse;
use vir_py_rs_type::ast::core::ASTNode;
use vir_py_rs_type::base::{Value, ValueContainer, ValueKind};
use vir_py_rs_type::builtin::Mapping;
use vir_py_rs_type::exec_ctx::ExecutionContext;

#[test]
fn test_simple_assignment_and_expr() {
    let module = parse!({
        a = 10;
        a = a + 5;
        a;
    });
    let arena_rc = Rc::new(RefCell::new(Bump::new()));
    let mut global_scope = Mapping { mapping: HashMap::new() };

    let initial_value: Value<'static> = {
        let arena_borrow: Ref<Bump> = arena_rc.borrow();
        let arena_ref: &Bump = &arena_borrow;
        let long_lived_arena: &'static Bump = unsafe { std::mem::transmute(arena_ref) };
        ValueContainer::new(ValueKind::None, long_lived_arena)
    };

    global_scope.mapping.insert("a".to_string(), Rc::new(RefCell::new(initial_value)));

    let mapping = vec![Rc::new(RefCell::new(global_scope))];
    let ctx = Rc::new(RefCell::new(ExecutionContext::new(arena_rc.clone(), 1000, mapping.clone())));

    let result = module.eval(ctx);

    assert!(result.is_ok(), "Evaluation failed: {:?}", result.err());

    let value = (&mapping).get(0).unwrap().borrow().mapping.get("a").unwrap().borrow().kind.clone();

    match value {
        ValueKind::Int(i) => assert_eq!(i.value, 15),
        _ => panic!("Expected an integer result, but got {:?}", value),
    }
}
