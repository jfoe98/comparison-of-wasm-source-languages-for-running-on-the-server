package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	startup()

	input := os.Args[2]
	input_as_int, err := strconv.Atoi(input)

	if err != nil {
		panic(err)
	}

	filesplit(input_as_int)

	finish()
}

func filesplit(n int) {

	fmt.Println("Starting filesplit")

	inputFile, err := get_file(n)
	if err != nil {
		panic(err)
	}

	defer inputFile.Close()

	scanner := bufio.NewScanner(inputFile)

	for scanner.Scan() {
		number := scanner.Text()
		number_as_int, err := strconv.Atoi(number)

		if err != nil {
			panic(err)
		}

		err = write_to_new_file(number_as_int)
		if err != nil {
			panic(err)
		}
	}

	if err := scanner.Err(); err != nil {
		panic(scanner.Err())
	}
}

func write_to_new_file(number int) error {
	file, err := os.OpenFile(fmt.Sprintf("./%v.txt", number%10), os.O_RDWR|os.O_CREATE|os.O_APPEND, 0644)
	if err != nil {
		return err
	}

	defer file.Close()

	_, err = fmt.Fprintf(file, "%v\n", number)

	// no explicit flush needed, os.File is not buffered

	return err
}

func get_file(n int) (*os.File, error) {
	return os.Open(fmt.Sprintf("./numbers_%v.txt", n))
}

//export startup
func startup()

//export finish
func finish()
