package main

import "net"
import "fmt"
import "bufio"
import "strings"

const PORT string = "1234"

func main() {
	ln, _ := net.Listen("tcp", ":" + PORT)
	
	conn, _ := ln.Accept()
	for {
		message, _ := bufio.NewReader(conn).ReadString('\n')
		fmt.Print(string(message))
		newmessage := strings.ToUpper(message)
		conn.Write([]byte(newmessage + "\n"))
	}
}