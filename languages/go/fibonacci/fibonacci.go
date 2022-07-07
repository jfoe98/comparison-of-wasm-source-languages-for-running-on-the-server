package main

func main() {
	//empty by design
}

//Annotation for tiny-go to export function in wasm module
//export fibonacci
func fibonacci(n int32) int64 {
	startup()

	result := fibonacci_internal(n)

	finish()

	return result
}

func fibonacci_internal(n int32) int64 {
	if n == 0 {
		return 0
	}

	if n == 1 {
		return 1
	}

	return fibonacci_internal(n-1) + fibonacci_internal(n-2)
}

//export startup
func startup()

//export finish
func finish()
