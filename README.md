# miniblink-rs

Rust bindings to [mininlink49](https://github.com/weolar/miniblink49)

```rust
use miniblink::{app::AppBuilder, webview::WebViewBuilder};

fn main() {
    let app = AppBuilder::default().build().unwrap();

    let _webview = WebViewBuilder::default()
        .with_window_title("Hello, Miniblink")
        .with_url("https://example.com")
        .with_visible(true)
        .with_on_window_closing_handler(|_| {
            std::process::exit(0);
        })
        .build();

    app.run_message_loop();
}
```

See [examples](./miniblink/examples) for basic usage

# Notes

This project is now under development, not ready for production.

The api in rust safe wrapper `./miniblink` may changed in the future.

The crate `./miniblink-sys` is generated using `bindgen`.

The rust safe wrapper is not aimed to provide a complete API set. But if you need to wrap a specified API, create an issue or pull a request.

Thank you!

# Credits

https://github.com/weolar/miniblink49
https://github.com/tauri-apps/wry
https://github.com/fltk-rs/fltk-rs

# License

Apache-2.0/MIT
