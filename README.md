# Simple HTTP proxy demo

This is a reverse proxy server.

The data that is sent through this server goes to another server and back to the client.

It's configurable via `config.toml` file.

![Gif demo](/demo/recording.gif?raw=true "Making requests through the proxy")

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
