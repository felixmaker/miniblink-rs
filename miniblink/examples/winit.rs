use miniblink::{app::AppBuilder, webview::WebViewBuilder};
use raw_window_handle::HasWindowHandle;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let _app = AppBuilder::default()
        .with_dpi_support(true)
        .build()
        .unwrap();

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
                WindowEvent::Resized(size) => webview.set_size(size.width, size.height),
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
