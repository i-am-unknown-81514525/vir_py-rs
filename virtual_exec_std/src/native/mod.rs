use virtual_exec_extern::import_parse;
use virtual_exec_extern::module_extern::*;

macro_rules! load_vel {
    (std) => {
        import_parse!(relative!("native_std/std.vel"))
    };
}
fn _a() {
    import_parse!(relative!("src/native_std/std.vel"));
    relative!("src/native_std/std.vel");
}