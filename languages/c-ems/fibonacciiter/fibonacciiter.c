#include <stdio.h>
#include <emscripten/emscripten.h>

extern void startup();
extern void finish();

EMSCRIPTEN_KEEPALIVE int64_t fibonacciiter(int32_t n) {
    startup();

    if (n == 0) {
		return 0;
	}

	if (n == 1) {
		return 1;
	}

	int64_t secondLast = 0;
	int64_t last = 1;
	int64_t fib = 0;

    int i = 1;
	while (i < n) {
        fib = last + secondLast;
        secondLast = last;
        last = fib;
        i++;
    }

    finish();

    return fib;
}