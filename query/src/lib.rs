use rustpython;

fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Ok(())
}

getrandom::register_custom_getrandom!(custom_getrandom);

#[ic_cdk_macros::query]
fn simple_query() -> String {
    let interpreter = rustpython::vm::Interpreter::without_stdlib(Default::default());

    let result = interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        vm.run_code_string(
            scope.clone(),
            r#"
def hello_world():
    return "Hello Dan and Ben!"

result = hello_world()
            "#,
            "".to_owned(),
        )
        .unwrap();

        let result_value = scope.globals.get_item("result", vm).unwrap();
        let result_string: String = result_value.try_into_value(vm).unwrap();

        result_string
    });

    result
}
