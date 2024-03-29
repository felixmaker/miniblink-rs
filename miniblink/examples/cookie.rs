use miniblink::{app, webview::{WebViewBuilder, WebViewGetter}};

fn main() {
    app::initialize("node.dll").unwrap();

    let _ = WebViewBuilder::default()
        .with_window_title("Hello, Miniblink")
        .with_url("https://u.4399.com/login.html")
        .with_visible(true)
        .with_on_window_closing_handler(|_| {
            std::process::exit(0);
        })
        .with_on_document_ready_handler(|wv| {
            let cookie = wv.get_cookie();
            if cookie.contains("Uauth=") {
                println!("User logined!");
                println!("{}", cookie);
                // do something...
            }
        })
        .build();

    app::run_message_loop();
}
