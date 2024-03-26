use miniblink::{app::AppBuilder, webview::WebViewBuilder};
use raw_window_handle::HasWindowHandle;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

#[derive(Debug)]
enum UserEvent {
    TitleChanged(String),
}

fn main() {
    let _app = AppBuilder::default()
        .with_dpi_support(true)
        .build()
        .unwrap();

    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::with_user_event().build().unwrap();
    let event_proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let window_handle = window.window_handle().unwrap();

    let webview = WebViewBuilder::default()
        .with_parent(&window_handle)
        .with_url("https://bing.com")
        .with_on_title_changed_handler(move |_, title| {
            event_proxy
                .send_event(UserEvent::TitleChanged(title))
                .unwrap();
        })
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
            Event::UserEvent(event) => match event {
                UserEvent::TitleChanged(title) => window.set_title(title.as_str()),
            },
            _ => {}
        })
        .unwrap();
}
