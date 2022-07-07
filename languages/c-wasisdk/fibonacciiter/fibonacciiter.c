#include <stdio.h>
#include <stdlib.h>
#include "declarations.h"

#define __IMPORT(name) \
    __attribute__((__import_module__("env"), __import_name__(#name)))

void startup() __IMPORT(startup);
void finish() __IMPORT(finish);

int main(int argc, char *argv[]) {
    startup();
    
    int64_t n = atoi(argv[2]);

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
}
