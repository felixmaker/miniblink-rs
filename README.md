# miniblink-rs

Rust bindings to [mininlink49](https://github.com/weolar/miniblink49)

```rust
use miniblink::{app, webview::*};

fn main() {
    app::init("mb.dll").unwrap();
    let view = WebView::default();
    view.load_url("https://example.com");
    view.on_close(|_| std::process::exit(0));
    view.show();
    app::run_message_loop();
}
```

See [examples](./miniblink/examples) for basic usage.

# Notes

This project is now under development, not ready for production.

The crate `./miniblink-sys` is generated using `bindgen`.

The rust safe wrapper `./miniblink` is not aimed to provide a complete API set.

For now, the basic api in rust safe wrapper `./miniblink` is more stable than the earlier version, but still, may change in the future.

It takes time to wrapper all api, so PRs are welcomed! :D

From 0.3.x, api migrates from wkeXXX to mbXXX.

# Credits

https://github.com/weolar/miniblink49
https://github.com/tauri-apps/wry
https://github.com/fltk-rs/fltk-rs

# License

Apache-2.0/MIT
