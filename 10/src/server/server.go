package server

import "net"
import "fmt"
import "bufio"
import "strings"

// Start starts server
func Start(port string) {
	ln, _ := net.Listen("tcp", ":"+port)

	conn, _ := ln.Accept()
	for {
		message, _ := bufio.NewReader(conn).ReadString('\n')
		fmt.Print(string(message))
		newmessage := strings.ToUpper(message)
		conn.Write([]byte(newmessage + "\n"))
	}
}
