package main

import "net"
import "fmt"
import "bufio"
import "os"

const HOST string = "localhost"
const PORT string = "1234"

func main() {
	
	conn, _ := net.Dial("tcp", HOST + ":" + PORT)
	for {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("> ")
		text, _ := reader.ReadString('\n')
		fmt.Fprintf(conn, text + "\n")
		message, _ := bufio.NewReader(conn).ReadString('\n')
		fmt.Print(message)
	}
}