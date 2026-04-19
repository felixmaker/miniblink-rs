use miniblink::{app, params::JsQueryResult, webview::*};

fn main() {
    app::init("./mb.dll").unwrap();
    let view = WebView::default();
    view.on_query(|_, params| match params.custom_message {
        0 => JsQueryResult {
            custom_message: 0,
            response: format!("hello, {}", params.request),
        },
        _ => JsQueryResult {
            custom_message: -1,
            response: "".into(),
        },
    });
    view.on_close(|_| std::process::exit(0));
    view.move_to_center();
    view.load_html_with_base_url(
        r#"
    <html>
    <head>
    <title>Hello, world!</title>        
    </head>
    <body>
    <input id="input1"></input>
    <input id="input2" disabled></input>
    <button onclick="say_hello();">Hello</button>
    <a href="https://miniblink.net" target="_blank">Miniblink</a>
    <a href="https://www.whatismybrowser.com/detect/what-http-headers-is-my-browser-sending/" target="_blank">Whatismybrowser</a>
    <script>
    var say_hello = function(){
        var input1=document.getElementById('input1');
        var input2=document.getElementById('input2');
        window.mbQuery(0, input1.value, function(message, response) {
            input2.value=response;
        });
    }
    </script>
    </body>
    </html>        
    "#,
        "",
    );
    view.on_create_view(|_, params| {
        let window = WebView::default();
        window.load_url(&params.url);
        window.show();
        Some(window)
    });
    view.show();
    app::run_message_loop();
}
