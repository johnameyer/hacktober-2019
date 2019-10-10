make: server client

server:	server.go
	go build -o $@ $^

client:	client.go
	go build -o $@ $^

clean:
	rm client server