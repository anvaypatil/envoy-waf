```rust
fn send_response(&self) {
    match self.get_http_request_header(":path") {
        Some(path) if path == "/hello" => {
            self.send_http_response(
                200,
                vec![("Hello", "World"), ("Powered-By", "proxy-wasm")],
                Some(b"Hello, World!\n"),
            );
            return Action::Pause;
        }
        _ => Action::Continue,
    }
}
```