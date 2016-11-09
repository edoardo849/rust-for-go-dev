package main

import "fmt"

func main() {

	fmt.Println("Hello Manchester!")
}

func mutable() {

	x := 1
	fmt.Printf("X is %d\n", x)

	x = 2
	fmt.Printf("X is %d\n", x)

}
