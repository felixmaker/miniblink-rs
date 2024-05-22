# miniblink

Rust safe bindings to miniblink49

It's now under development, not ready for production.

The api in this crate may change in the future.

```
use miniblink::prelude::*;
use miniblink::{app, webview::*};

fn main() {
    app::init("mb.dll").unwrap();
    let view = WebView::default();
    view.move_to_center();
    view.load_url("https://example.com");
    view.on_close(|_| std::process::exit(0));
    view.show();
    app::run_message_loop();
}
```