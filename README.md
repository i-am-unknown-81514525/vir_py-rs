# VirtualExec

A rust library to perform sandboxed safe expression evaluation, in a similar syntax to rust

```rust
use virtual_exec_core::{Machine, parse, compile};
use virtual_exec_core::sequential::exec::State;
use virtual_exec_type::error::ExecutionError;
use virtual_exec_type::mem::OwnedValue;

#[test]
fn test_simple_assignment() {
  let code = "a = 1; b = 2; c = 3; if a != b {d = 2;} d += d; d;";
  let compiled = compile(&parse(code).unwrap());
  println!("{:?}", compiled);
  let mut machine = Machine::new(compiled, 100, 100, vec![]).unwrap();
  match machine.sync_run_all() {
    Ok(State::Ok) => {},
    Ok(reason) => {
      println!("Machine: {:?}, state: {:?}", machine, reason);
    },
    Err(e) => {
      println!("Machine: {:?}, err: {:?}", machine, e);
    }
  }
  assert_eq!(machine.get("a"), Some(OwnedValue::Int(1)));
  assert_eq!(machine.get("d"), Some(OwnedValue::Int(4)));
}


#[test]
fn test_fn() {
  let code = "a = 10;
        fn add(a, b) {
            return a + b;
        }
        while a > 0 {
            a = add(a, -1);
        }";
  let compiled = compile(&parse(code).unwrap());
  println!("{:?}", compiled);
  let mut machine = Machine::new(compiled, 100, 1000, vec![]).unwrap();
  match machine.sync_run_all() {
    Ok(State::Ok) => {},
    Ok(reason) => {
      println!("Machine: {:?}, state: {:?}", machine, reason);
    },
    Err(e) => {
      println!("Machine: {:?}, err: {:?}", machine, e);
    }
  }
  assert_eq!(machine.get("a"), Some(OwnedValue::Int(0)));
}
```
An example if the execution. In particular, the `100` and `1000` in the `test_fn` defines the memory and lifetime of the machine respectively, 
which this allowed up to 1000 operation and 100 virtual byte allocation, and would raise stop with Ok(State::Timeout) if it take longer than that.

Compile have also been added to convert code into a linear instruction

```rust
use virtual_exec_parser::parser::parse;
use virtual_exec_parser::sequential::compile::compile;
use virtual_exec_parser::sequential::instructions::Instruction;

#[test]
fn test_value_creation_and_downcast() {
    let code = "a = 1; b = 2; c = 3; if a != b {d = 2;} d += d; d;";
    let parsed = parse(code).unwrap();
    let compiled = compile(&parsed);
    assert_eq!(compiled, vec![
        Instruction::LoadName(Box::from("a")),
        Instruction::LoadLitInt(1),
        Instruction::Assign,
        Instruction::LoadName(Box::from("a")),
        Instruction::LoadLitInt(2),
        Instruction::Assign,
        Instruction::LoadName(Box::from("c")),
        Instruction::LoadLitInt(3),
        Instruction::Assign,
        Instruction::LoadName(Box::from("a")),
        Instruction::LoadName(Box::from("b")),
        Instruction::NotEq,
        Instruction::JmpZ(16),
        Instruction::LoadName(Box::from("d")),
        Instruction::LoadLitInt(2),
        Instruction::Assign,
        Instruction::LoadName(Box::from("d")),
        Instruction::LoadName(Box::from("d")),
        Instruction::LoadName(Box::from("d")),
        Instruction::Add,
        Instruction::Assign,
        Instruction::LoadName(Box::from("d")),
        Instruction::Pop
    ]);
}

```

### Example code:
```
print = std.print;
println = std.println;

i = 42;
f = 3.5;
s = "hi\n";
b = true;
n = None;
xs = [10, 20, 30];

a = i + 8;
a -= 5;
a *= 2;
a /= 3;
a = int(a);
a %= 7;
a <<= 4;
a >>= 1;
a |= 1;
a &= 0xFF;
println(a);
println(-i);
println(!b);
println(i > 10 && f <= 4.0);
println(i < 0 || f > 1.0);
println(i == 42);

if a > 100 {
    println("big");
} else if a > 10 {
    println("medium");
} else {
    println("small");
}

k = 0;
total = 0;
while k < 5 {
    total += k;
    k += 1;
}
println(total);

fn add(x, y) {
    return x + y;
}

fn fib(x) {
    if x < 2 {
        return x;
    }
    return fib(x - 1) + fib(x - 2);
}

println(add(2, 3));
println(fib(10));

xs[0] = 99;
println(xs[1]);
println(to_str(xs));

arr = create_array();
push_array(arr, 1);
push_array(arr, "two");
println(arr_get_len(arr));
println(arr_get_from_idx(arr, 1));
println(pop_array(arr));

obj = create_obj();
obj.name = "virtual_exec";
obj["count"] = 3;
println(obj.name);
println(to_str(dir(obj)));
println(rm_ele(obj, "count"));
println(to_str(obj));

println(concat("total = ", to_str(total)));
println(int(3.9));

out = get_output_stream();
write_stream(out, "stream\n");
print("no newline -> ");
println("newline");
```

WIP Feature list:
- [x] Variable assignment
- [x] Attribute assignment
- [x] Subscript assignment (i.e. `x[a]`)
- [x] Expression evaluation
- [x] A parser and type system
- [x] Attribute system
- [x] Function call
- [x] `while` loop
- [ ] `for` loop
- [x] FFI function (Calling rust function from sandbox code with custom lifetime consumption)
- [x] Function definition
- [x] `if` statement
- [ ] Custom object definition
- [x] Use `await` in rust to allow context switching to other part of program to make it not blocking
  - [x] Switch to async-agnostic `Arc`, `Mutex`, `RwLock`
- [x] Linear instruction system (this allows `await` system later)
- [ ] `try` `catch` with stack unwinding and memory allocation recalculation
- [ ] literal Collections and object creation
- [x] Support for any object
- [ ] Support for instruction, state and memory exporting (Only on value that can be converted to OwnedValue)
- [ ] WASM handling?
- [ ] `__private` for path resolution from proc macro

### Sub-crate List:
- [virtual_exec_type](https://crates.io/crates/virtual_exec_type)
- [virtual_exec_parser](https://crates.io/crates/virtual_exec_parser)
- [virtual_exec_macro](https://crates.io/crates/virtual_exec_macro)
- [virtual_exec_core](https://crates.io/crates/virtual_exec_core)
- [virtual_exec_extern](https://crates.io/crates/virtual_exec_extern)
- [virtual_exec_std](https://crates.io/crates/virtual_exec_std)
- [virtual_exec_repl](https://crates.io/crates/virtual_exec_repl)
- [virtual_exec_js](https://crates.io/crates/virtual_exec_js)

### Video Demo

https://github.com/user-attachments/assets/9a15c9ba-6932-466f-8d96-412dca2aa888
