Inter-Process Communication (IPC) for client-server applications can be
accomplished using
[UNIX sockets][unix-sockets].

Both client and server need to use the same path for the socket.

{common.rs}

The client program:

{client.rs}

The server program:

{server.rs}

Let's test the programs

```
$ rustc client.rs; rustc server.rs

# Terminal 1
$ ./server
Server started, waiting for clients

# Terminal 2
$ ./client hello

# Terminal 1
Server started, waiting for clients
Client said: hello
```

[unix-sockets]: http://en.wikipedia.org/wiki/Unix_domain_socket
