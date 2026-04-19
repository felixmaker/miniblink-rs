use miniblink::{app, webview::*};

fn main() {
    app::init("./mb.dll").unwrap();
    let view = WebView::default();
    view.on_close(|_| std::process::exit(0));
    view.load_url("https://miniblink.net/");
    view.show();

    app::run_message_loop();
}
