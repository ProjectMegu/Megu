use aidan_rs::{module::Module, types::{Type, TypeKind}};

#[test]
fn test_module() {
    let module = Module::new();
    module.print();
}
