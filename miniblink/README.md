# miniblink

Rust safe bindings to miniblink49

It's now under development, not ready for production.

The api in this crate may change in the future.

```
use miniblink::prelude::*;
use miniblink::{app, webview::WebView};

fn main() {
    app::initialize("node.dll").unwrap();

    let wv = WebView::default();
    wv.load_url("https://example.com");
    wv.show_window(true);

    wv.on_window_closing(|_| {
        std::process::exit(0);
    });

    app::run_message_loop();
}
```