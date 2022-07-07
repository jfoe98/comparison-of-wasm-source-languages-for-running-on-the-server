import { finish, startup } from "./env";

export function fibonacci(n: i32): i64 {
    startup();

    let result = fibonacciInternal(n);

    finish();

    return result;
}

function fibonacciInternal(n: i32): i64 {
    if (n == 0) {
        return 0;
    }

    if (n == 1) {
        return 1;
    }

    return fibonacciInternal(n-1) + fibonacciInternal(n-2)
} 