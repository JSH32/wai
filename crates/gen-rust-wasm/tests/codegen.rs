#![allow(dead_code, type_alias_bounds)]

#[test]
fn ok() {}

#[rustfmt::skip]
mod imports {
    test_helpers::codegen_rust_wasm_import!(
        "*.wai"

        // If you want to exclude a specific test you can include it here with
        // gitignore glob syntax:
        //
        // "!wasm.wai"
        // "!host.wai"
        //
        //
        // Similarly you can also just remove the `*.wai` glob and list tests
        // individually if you're debugging.
    );
}

mod exports {
    test_helpers::codegen_rust_wasm_export!(
        "*.wai"

        // TODO: these use push/pull buffer which isn't implemented in the test
        // generator just yet
        "!wasi-next.wai"
        "!host.wai"
    );
}
