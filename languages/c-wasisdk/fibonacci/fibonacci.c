#include <stdio.h>
#include <stdlib.h>
#include "declarations.h"

#define __IMPORT(name) \
    __attribute__((__import_module__("env"), __import_name__(#name)))

void startup() __IMPORT(startup);
void finish() __IMPORT(finish);

int64_t fibonacci(int32_t n) {
    if(n == 0){
        return 0;
    } else if(n == 1) {
        return 1;
    } else {
        return (fibonacci(n-1) + fibonacci(n-2));
    }
}

int main(int argc, char *argv[]) {
    startup();
    
    int64_t result = fibonacci(atoi(argv[2]));

    finish();
}
