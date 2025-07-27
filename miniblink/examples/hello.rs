use miniblink::{app, webview::*};

fn main() {
    app::init("./mb.dll").unwrap();
    let view = WebViewWindow::default();
    view.load_url("https://miniblink.net");
    view.on_close(|| std::process::exit(0));
    view.show();
    app::run_message_loop();
}
