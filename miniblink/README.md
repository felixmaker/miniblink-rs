# miniblink

Rust safe bindings to miniblink49

It's now under development, not ready for production.

The api in this crate may change in the future.

```
use miniblink::{app, webview::WebViewBuilder};

fn main() {
    app::init("node.dll");
    app::bind("hello", |x| format!("Hello, {x}"));

    let _webview = WebViewBuilder::default()
        .with_window_title("Hello, Miniblink")
        .with_html(
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
            input2.value=window.hello(input1.value);
        }
        </script>
        </body>
        <html>        
        "#,
        )
        .with_visible(true)
        .with_on_window_closing_handler(|_| {
            std::process::exit(0);
        })
        .build();

    app::run_message_loop();
}
```