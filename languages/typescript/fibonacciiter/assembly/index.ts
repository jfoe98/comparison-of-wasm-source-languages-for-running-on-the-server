import { finish, startup } from "./env";

export function fibonacciiter(n: i32): i64 {
    startup();

    if (n == 0) {
        return 0;
    }

    if (n == 1) {
        return 1;
    }

    let secondLast: i64 = 0;
    let last: i64 = 1;
    let fib: i64 = 0;

    let i: i32 = 1;
    while (i < n) {
        fib = last + secondLast;
        secondLast = last;
        last = fib;
        i++;
    }

    finish();

    return fib;
}