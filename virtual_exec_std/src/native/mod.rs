use virtual_exec_macro::import_compile_relative;

macro_rules! load_vel {
    (std) => {
        import_compile_relative!("src/native_std/std.vel")
    };
}
fn _a() {
    load_vel!(std);
}