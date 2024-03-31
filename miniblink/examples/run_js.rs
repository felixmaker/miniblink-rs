use miniblink::prelude::*;
use miniblink::{app, webview::WebView};

fn main() {
    app::initialize("node.dll").expect("Failed to initialize miniblink!");

    let wv = WebView::default();

    wv.load_html("<html></html>");
    wv.on_document_ready(|wv| {
        let result = wv.eval::<i32>("{return 1}");
        println!("{:?}", result);
    });

    wv.on_window_closing(|_| {
        std::process::exit(0);
    });

    app::run_message_loop();
}
