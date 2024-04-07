use std::mem::size_of;

use miniblink::{
    app,
    types::{MouseFlags, MouseMessage},
    webview::WebView,
};
use miniblink_sys::HWND;
use raw_window_handle::HasWindowHandle;
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteObject, GetDC, ReleaseDC, SelectObject,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HDC, SRCCOPY,
};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

#[derive(Debug)]
enum UserEvent {
    TitleChanged(String),
}

struct Render {
    view: HWND,
    bitmap: HBITMAP,
    hdc: HDC,
    width: i32,
    height: i32,
}

impl Render {
    fn init(view: HWND) -> Self {
        let hdc = unsafe { CreateCompatibleDC(HDC::default()) };
        Self {
            view,
            bitmap: HBITMAP::default(),
            hdc,
            width: 800,
            height: 600,
        }
    }
    fn render_on_blink_paint(
        &mut self,
        wv: &mut WebView,
        blinkdc: HDC,
        x: i32,
        y: i32,
        _cx: i32,
        _cy: i32,
    ) {
        self.width = wv.get_width();
        self.height = wv.get_height();
        self.create_bitmap();
        let window_hdc = unsafe { GetDC(self.view) };
        let _ = unsafe {
            BitBlt(
                self.hdc,
                x,
                y,
                self.width,
                self.height,
                blinkdc,
                x,
                y,
                SRCCOPY,
            )
        };
        let _ = unsafe {
            BitBlt(
                window_hdc,
                x,
                y,
                self.width,
                self.height,
                self.hdc,
                x,
                y,
                SRCCOPY,
            )
        };
        unsafe { ReleaseDC(self.view, window_hdc) };
    }

    fn create_bitmap(&mut self) {
        let bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: self.width,
                biHeight: -self.height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let hbmp = unsafe {
            CreateDIBSection(
                HDC::default(),
                &bitmap_info,
                DIB_RGB_COLORS,
                std::ptr::null_mut(),
                HWND::default(),
                0,
            )
        }
        .unwrap();

        unsafe { SelectObject(self.hdc, hbmp) };

        if !self.bitmap.is_invalid() {
            unsafe { DeleteObject(self.bitmap) };
        }

        self.bitmap = hbmp;
    }
}

fn main() {
    // Require `rwh_06`` feature.
    app::initialize("node.dll").unwrap();
    app::enable_high_dpi_support();

    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::with_user_event().build().unwrap();
    let event_proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let window_handle = {
        if let raw_window_handle::RawWindowHandle::Win32(hwnd) =
            window.window_handle().unwrap().as_raw()
        {
            hwnd.hwnd
        } else {
            unimplemented!()
        }
    };

    let hwnd = HWND(window_handle.get());
    let webview = WebView::create_web_view();
    webview.resize(800, 600);
    webview.set_handle(hwnd);
    webview.show_window(true);
    webview.load_url("https://bing.com");
    webview.on_title_changed(move |_, title| {
        event_proxy
            .send_event(UserEvent::TitleChanged(title))
            .unwrap();
    });

    let mut render = Render::init(hwnd);

    webview.on_paint_updated(move |wv, hdc, x, y, cx, cy| {
        render.render_on_blink_paint(wv, hdc, x, y, cx, cy)
    });

    event_loop.set_control_flow(ControlFlow::Wait);
    let mut current_pos = PhysicalPosition::default();
    event_loop
        .run(|event, flow| match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => flow.exit(),
                WindowEvent::Resized(size) => webview.resize(size.width as i32, size.height as i32),
                WindowEvent::MouseInput { button, .. } => match button {
                    winit::event::MouseButton::Left => {
                        webview.fire_mouse_event(
                            MouseMessage::LBUTTONDOWN,
                            current_pos.x as i32,
                            current_pos.y as i32,
                            MouseFlags::LBUTTON,
                        );
                    }
                    _ => {}
                },
                WindowEvent::CursorMoved { position, .. } => {
                    current_pos = position;
                    webview.fire_mouse_event(
                        MouseMessage::MOUSEMOVE,
                        current_pos.x as i32,
                        current_pos.y as i32,
                        MouseFlags::SHIFT,
                    );
                }
                _ => {}
            },
            Event::UserEvent(event) => match event {
                UserEvent::TitleChanged(title) => window.set_title(title.as_str()),
            },
            _ => {}
        })
        .unwrap();
}
