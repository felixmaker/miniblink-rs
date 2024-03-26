use miniblink::{app::AppBuilder, webview::WebViewBuilder};

fn main() {
    let app = AppBuilder::default()
        .build()
        .expect("Failed to initialize miniblink!");

    let _ = WebViewBuilder::default()
        .with_html("<html></html>")
        .with_on_document_ready_handler(|wv| {
            let result = wv.run_js::<String>("{return 1}");
            println!("{:?}", result);
        })
        .with_on_window_closing_handler(|_| {
            std::process::exit(0);
        })
        .build();

    app.run_message_loop();
}
