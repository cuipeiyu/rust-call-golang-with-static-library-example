package main

import "C"

func main() {}

//export Ping
func Ping() *C.char {
	return C.CString("pong")
}
