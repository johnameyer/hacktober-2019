package client

import "net"
import "fmt"
import "bufio"
import "os"

// Start starts the client
func Start(host string, port string) {

	conn, _ := net.Dial("tcp", host+":"+port)
	for {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("> ")
		text, _ := reader.ReadString('\n')
		fmt.Fprintf(conn, text+"\n")
		message, _ := bufio.NewReader(conn).ReadString('\n')
		fmt.Print(message)
	}
}
