use miniblink::{app, webview::WebView};

fn main() {
    app::initialize("node.dll").unwrap();

    let wv = WebView::new(0, 0, 800, 600);
    wv.set_window_title("Hello, Miniblink");
    wv.load_url("https://u.4399.com/login.html");
    wv.show_window(true);

    wv.on_window_closing(|_| {
        std::process::exit(0);
    });

    wv.on_document_ready(|wv| {
        let cookie = wv.get_cookie();
        if cookie.contains("Uauth=") {
            println!("User logined!");
            println!("{}", cookie);
            // do something...
        }
    });

    app::run_message_loop();
}
