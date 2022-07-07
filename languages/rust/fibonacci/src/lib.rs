#[no_mangle]
pub extern "C" fn fibonacci(n: i32) -> i64 {
    unsafe {
        startup();
    }
    
    let result = fibonacci_internal(n);

    unsafe {
        finish();
    }

    result
}

fn fibonacci_internal(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_internal(n - 1) + fibonacci_internal(n - 2),
    }
}

#[link(wasm_import_module = "env")]
extern "C" {
    fn startup();
    fn finish();
}