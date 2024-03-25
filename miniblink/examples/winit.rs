use miniblink::{app::AppBuilder, webview::WebViewBuilder};
use raw_window_handle::HasWindowHandle;
use winit::event_loop::EventLoop;

fn main() {
    let _app = AppBuilder::default()
        .with_dpi_support(true)
        .build()
        .unwrap();

    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let window_handle = window.window_handle().unwrap();

    let webview = WebViewBuilder::default()
        .with_parent(&window_handle)
        .with_url("http://example.com")
        .with_visible(true)
        .with_on_window_closing_handler(|_| {
            std::process::exit(0);
        })
        .build()
        .unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    event_loop
        .run(|event, flow| match event {
            winit::event::Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                winit::event::WindowEvent::CloseRequested => flow.exit(),
                winit::event::WindowEvent::Resized(size) => {
                    webview.set_size(size.width, size.height)
                }
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
