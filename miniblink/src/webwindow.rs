use std::{
    cell::RefCell,
    ffi::CString,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    call_api_or_panic,
    content::{set_webwindow_handler, WEBVIEW_CONTENT},
    types::WindowType,
    webview::WebView,
};

/// The webview window.
pub struct WebViewWindow {
    inner: WebView,
}

impl WebViewWindow {
    /// Create a new webview window.
    pub fn new(typ: WindowType, x: i32, y: i32, width: i32, height: i32) -> Self {
        let inner = unsafe {
            call_api_or_panic().mbCreateWebWindow(
                typ as _,
                std::ptr::null_mut(),
                x,
                y,
                width,
                height,
            )
        };
        let webview = Self {
            inner: unsafe { WebView::from_raw(inner) },
        };
        set_webwindow_handler(&webview);
        webview
    }

    /// Set close callback.
    pub fn on_close<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView) -> bool + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_close = Some(callback);
        });
    }

    /// Set destroy callback.
    pub fn on_destroy<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView) -> bool + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_destroy = Some(callback);
        });
    }

    /// Show the window.
    pub fn show(&self) {
        unsafe {
            call_api_or_panic().mbShowWindow(self.as_ptr(), 1);
        }
    }

    /// Hide the window.
    pub fn hide(&self) {
        unsafe {
            call_api_or_panic().mbShowWindow(self.as_ptr(), 0);
        }
    }

    /// Resize the window.
    pub fn resize(&self, width: i32, height: i32) {
        unsafe { call_api_or_panic().mbResize(self.as_ptr(), width, height) }
    }

    /// Move the window.
    pub fn move_window(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            call_api_or_panic().mbMoveWindow(self.as_ptr(), x, y, width, height);
        }
    }

    /// Move the window to center.
    pub fn move_to_center(&self) {
        unsafe {
            call_api_or_panic().mbMoveToCenter(self.as_ptr());
        }
    }

    /// Set the window title.
    pub fn set_window_title(&self, title: &str) {
        let title = CString::new(title).unwrap();
        unsafe { call_api_or_panic().mbSetWindowTitle(self.as_ptr(), title.as_ptr()) }
    }
}

impl Deref for WebViewWindow {
    type Target = WebView;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for WebViewWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Default for WebViewWindow {
    fn default() -> Self {
        let window = Self::new(WindowType::Popup, 0, 0, 800, 600);
        window.move_to_center();
        window
    }
}
