use miniblink::{app, webview::*};

fn main() {
    app::init("./mb.dll").unwrap();
    let view = WebView::default();
    view.load_url("https://example.com");
    view.on_close(|_| std::process::exit(0));
    view.show();
    app::run_message_loop();
}
