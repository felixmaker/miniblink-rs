use miniblink::{
    app,
    webview::{WebViewBuilder, WebViewOperation},
};
use raw_window_handle::HasWindowHandle;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    app::initialize("node.dll").unwrap();
    app::enable_high_dpi_support();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Hello, Winit and Miniblink")
        .build(&event_loop)
        .unwrap();

    let window_handle = window.window_handle().unwrap();

    let webview = WebViewBuilder::default()
        .with_parent(&window_handle)
        .with_url("http://example.com")
        .with_visible(true)
        .build()
        .unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run(|event, flow| match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => flow.exit(),
                WindowEvent::Resized(size) => webview.resize(size.width as i32, size.height as i32),
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
