use miniblink::prelude::*;
use miniblink::{app, webview::WebView};
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
    app::initialize("node.dll").unwrap();
    app::enable_high_dpi_support();

    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::with_user_event().build().unwrap();
    let event_proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let window_handle = window.window_handle().unwrap();

    let webview = WebView::new_as_child(&window_handle, 0, 0, 800, 600).unwrap();
    webview.show_window(true);
    webview.load_url("https://bing.com");
    webview.on_title_changed(move |_, title| {
        event_proxy
            .send_event(UserEvent::TitleChanged(title))
            .unwrap();
    });

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
            Event::UserEvent(event) => match event {
                UserEvent::TitleChanged(title) => window.set_title(title.as_str()),
            },
            _ => {}
        })
        .unwrap();
}
