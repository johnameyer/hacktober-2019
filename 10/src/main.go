package main

import (
	"os"
	"server"
	"client"
	"fmt"
)

// DefaultPort the port to use if none is specified
const DefaultPort string = "41000"

func usage(code int) {
	fmt.Printf("Usage:\t%s [-s | -c HOST | HOST] [PORT]\n", os.Args[0])
	
	os.Exit(code)
}

func main() {
	var port string = DefaultPort
	var host string
	var runServer bool = false

	if len(os.Args) >= 2 && os.Args[1][0] == '-' {
		if os.Args[1] == "-s" {
			runServer = true
			if len(os.Args) == 3 {
				port = os.Args[2]
			} else if len(os.Args) != 2 {
				usage(1)
			} 
		} else if os.Args[1] == "-c" {
			if len(os.Args) == 3 || len(os.Args) == 4 {
				host = os.Args[2]
				if len(os.Args) == 4 {
					port = os.Args[3]
				}
			} else {
				usage(1);
			}
		} else {
			usage(1)
		}
	} else {
		if len(os.Args) == 2 || len(os.Args) == 3 {
			host = os.Args[1]
			if len(os.Args) == 3 {
				port = os.Args[2]
			}
		} else {
			usage(1);
		}
	}

	if runServer {
		server.Start(port)
	} else {
		client.Start(host, port)
	}

}
