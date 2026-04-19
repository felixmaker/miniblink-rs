# miniblink-rs

Rust bindings to [mininlink49](https://github.com/weolar/miniblink49)

```rust
use miniblink::{app, webview::*};

fn main() {
    app::init("./mb.dll").unwrap();
    let view = WebView::default();
    view.on_close(|_| std::process::exit(0));
    view.load_url("https://miniblink.net/");
    view.show();

    app::run_message_loop();
}
```

See [examples](./miniblink/examples) for basic usage.

# Notes

This project is now under development, not ready for production.

Most of the common used API have been wrapped. If you need to more, just open an issue.

PRs are welcomed! :D

From 0.3.x, api migrates from wkeXXX to mbXXX.

# Credits

https://github.com/weolar/miniblink49
https://github.com/tauri-apps/wry
https://github.com/fltk-rs/fltk-rs

# License

Apache-2.0/MIT
