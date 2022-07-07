package main

func main() {
	//empty by design
}

//Annotation for tiny-go to export function in wasm module
//export fibonacciiter
func fibonacciiter(n int32) int64 {
	startup()

	if n == 0 {
		return 0
	}

	if n == 1 {
		return 1
	}

	var secondLast int64 = 0
	var last int64 = 1
	var fib int64 = 0

	var i int32 = 1
	for i < n {
		fib = last + secondLast
		secondLast = last
		last = fib
		i++
	}

	finish()

	return fib
}

//export startup
func startup()

//export finish
func finish()
