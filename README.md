# Simple HTTP proxy demo

This is a proxy server.

The data that is sent through this server goes to another server.

It's configurable via `config.toml` file.
This server also cuts off data that does not need to be forwarded.

# Try it out!

1. Install [Rust](https://rustup.rs/)
2. Start the server:
```bash
$ cargo run --release
```
3. Send HTTP requests through the server address to forward them to a remote server:
```
# this will return the response from `127.0.0.1:8081/hello` server endpoint. 
$ curl 127.0.0.1:8080/hello
```
