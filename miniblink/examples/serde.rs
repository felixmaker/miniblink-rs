use miniblink::{
    app::{AppBuilder, AppExt},
    webview::WebViewBuilder,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

fn main() {
    let app = AppBuilder::default().build().unwrap();

    app.bind("format_user", |user: User| {
        format!("{}: {}", user.name, user.age)
    });

    let _ = WebViewBuilder::default()
        .with_window_title("Hello, Miniblink")
        .with_html(
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
        )
        .with_visible(true)
        .with_on_window_closing_handler(|_| {
            std::process::exit(0);
        })
        .build();

    app.run_message_loop();
}
