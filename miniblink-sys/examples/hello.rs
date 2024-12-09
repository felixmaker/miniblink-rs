use miniblink_sys::*;

fn main() {
    unsafe {
        let lib = Library::new("mb.dll").unwrap();
        // let lib = Library::new("./miniblink.so").unwrap();
        let settings = std::mem::zeroed();
        lib.mbInit(&settings);
        let webview =
            lib.mbCreateWebWindow(MB_WINDOW_TYPE_POPUP, std::ptr::null_mut(), 20, 20, 200, 200);
        lib.mbLoadURL(webview, c"http://example.com".as_ptr());
        lib.mbShowWindow(webview, 1);
        lib.mbRunMessageLoop();
    }

    // app::js_bind_function(
    //     "hello",
    //     |es: JsExecState| -> MBResult<JsValue> {
    //         let arg1: String = es.arg_value(0)?;
    //         let result = format!("Hello, {}!", arg1);
    //         es.js_value(result)
    //     },
    //     1,
    // );

    // let wv = WebView::default();
    // wv.set_window_title("Hello, Miniblink");
    // wv.load_html(
    //     r#"
    //     <html>
    //     <head>
    //     <title>Hello, world!</title>
    //     </head>
    //     <body>
    //     <input id="input1"></input>
    //     <input id="input2" disabled></input>
    //     <button onclick="say_hello();">Hello</button>
    //     <script>
    //     var say_hello = function(){
    //         var input1=document.getElementById('input1');
    //         var input2=document.getElementById('input2');
    //         input2.value=window.hello(input1.value);
    //     }
    //     </script>
    //     </body>
    //     </html>
    //     "#,
    // );

    // wv.on_window_closing(|_| {
    //     std::process::exit(0);
    // });

    // wv.show_window(true);
    // app::run_message_loop();
}
