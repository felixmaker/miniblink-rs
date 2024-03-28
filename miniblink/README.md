# miniblink

Rust safe bindings to miniblink49

It's now under development, not ready for production.

The api in this crate may change in the future.

```
use miniblink::{app::AppBuilder, webview::WebViewBuilder};

fn main() {
    let app = AppBuilder::default()
        .with_lib_path("node.dll")
        .build()
        .expect("Failed to initialize miniblink!");

    let _ = WebViewBuilder::default()
        .with_window_title("Hello, Miniblink")
        .with_url("https://example.com")
        .with_on_window_closing_handler(|_| {
            std::process::exit(0);
        })
        .build();

    app.run_message_loop();
}
```