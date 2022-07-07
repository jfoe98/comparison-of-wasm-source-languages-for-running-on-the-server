#[no_mangle]
pub extern "C" fn fibonacciiter(n: i32) -> i64 {
    unsafe {
        startup();
    }
    
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut second_last:i64 = 0;
    let mut last:i64 = 1;
    let mut fib:i64 = 0;

    let mut i = 1;

    while i < n {
        fib = last + second_last;
        second_last = last;
        last = fib;
        i = i + 1;
    }

    unsafe {
        finish();
    }

    fib
}

#[link(wasm_import_module = "env")]
extern "C" {
    fn startup();
    fn finish();
}