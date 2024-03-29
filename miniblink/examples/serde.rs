use miniblink::{
    app,
    webview::{WebView, WebViewHandler, WebViewOperation},
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

fn main() {
    app::initialize("node.dll").unwrap();

    #[cfg(feature = "serde")]
    app::bind("format_user", |user: User| {
        Ok(format!("{}: {}", user.name, user.age))
    });

    let wv = WebView::default();
    wv.load_html(
        r#"
    <html>
    <body>
    <input id="name"></input>
    <input id="age"></input>
    <input id="msg" disabled></input>
    <button onclick="load_user();">Hello</button>
    <script>
    var load_user = function(){
        var name=document.getElementById('name');
        var age=document.getElementById('age');
        var msg=document.getElementById('msg');
        msg.value=window.format_user({name: name.value, age: parseInt(age.value)});
    }
    </script>
    </body>
    </html>        
    "#,
    );

    wv.show_window(true);
    wv.on_window_closing(|_| {
        std::process::exit(0);
    });

    app::run_message_loop();
}
