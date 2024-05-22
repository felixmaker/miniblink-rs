use miniblink::{app, webview::*};

fn main() {
    app::init("mb.dll").unwrap();
    let view = WebView::default();
    view.on_query(|_wv, msg, request| -> (QueryMessage, String) {
        match msg {
            0 => (0, format!("hello, {}", request)),
            _ => (-1, "".into()),
        }
    });
    view.on_close(|_wv| std::process::exit(0));
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
    view.show();
    app::run_message_loop();
}
