#include <stdio.h>
#include <emscripten/emscripten.h>

extern void startup();
extern void finish();

int64_t fibonacci_internal(int32_t n) {
    if(n == 0){
        return 0;
    } else if(n == 1) {
        return 1;
    } else {
        return (fibonacci_internal(n-1) + fibonacci_internal(n-2));
    }
}

EMSCRIPTEN_KEEPALIVE int64_t fibonacci(int32_t n) {
    startup();

    int64_t result = fibonacci_internal(n);

    finish();

    return result;
}