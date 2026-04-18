use std::{ffi::c_void, panic::AssertUnwindSafe};

use crate::call_api_or_panic;

// /// Defines all wrapped API.
// #[allow(dead_code)]
// pub(crate) enum Command {
//     StopLoading,
//     Reload,
//     GoBack,
//     GoForward,
//     Resize(i32, i32),
//     GetWindowHandle,
//     EditorSelectAll,
//     EditorUnSelect,
//     EditorCopy,
//     EditorCut,
//     EditorDelete,
//     EditorUndo,
//     EditorRedo,
//     EditorPaste,
//     SetCookie(CString, CString),
//     PerformCookieCommand(CookieCommand),
//     ClearCookie,
//     SetCookieJarPath(WideCString),
//     SetCookieJarFullPath(WideCString),
//     SetLocalStorageFullPath(WideCString),
//     FireMouseEvent(WindowMessage, i32, i32, MouseFlags),
//     FireMouseWheelEvent(WindowMessage, i32, i32, MouseFlags),
//     FireKeyUpEvent(VirtualKeyCode, KeyboardFlags, bool),
//     FireKeyDownEvent(VirtualKeyCode, KeyboardFlags, bool),
//     FireKeyPressEvent(u32, KeyboardFlags, bool),
//     SetFocus,
//     KillFocus,
//     RunJsSync(WebFrameHandle, CString, bool),
//     SetZoomFactor(f32),
//     GetZoomFactor,
//     SetHandle(WindowHandle),
//     SetHandleOffset(i32, i32),
//     SetUserAgent(CString),
//     LoadUrl(CString),
//     LoadHtmlWithBaseUrl(CString, CString),
//     EnableContextMenu(bool),
//     EnableCookie(bool),
//     EnableCspCheck(bool),
//     EnableDiskCache(bool),
//     EnableDragDrop(bool),
//     EnableDrag(bool),
//     EnableHeadless(bool),
//     EnableMemoryCache(bool),
//     EnableMouse(bool),
//     EnableNavigationToNewWindow(bool),
//     EnableNodeJs(bool),
//     EnableNpapiPlugins(bool),
//     EnableSystemTouch(bool),
//     EnableTouch(bool),
//     WindowShow(bool),
//     WindowResize(i32, i32),
//     WindowMove(i32, i32, i32, i32),
//     WindowMoveToCenter,
//     WindowSetTitle(CString),
// }

// pub(crate) fn invoke_command(webview: mbWebView, command: Command) {
//     use crate::command::Command::*;
//     match command {
//         LoadUrl(url) => {
//             unsafe {
//                 call_api_or_panic().mbLoadURL(webview, url.as_ptr());
//             };
//         }
//         LoadHtmlWithBaseUrl(html, base_url) => {
//             unsafe {
//                 call_api_or_panic().mbLoadHtmlWithBaseUrl(webview, html.as_ptr(), base_url.as_ptr())
//             };
//         }
//         StopLoading => unsafe {
//             call_api_or_panic().mbStopLoading(webview);
//         },
//         Reload => unsafe {
//             call_api_or_panic().mbReload(webview);
//         },
//         GoBack => unsafe {
//             call_api_or_panic().mbGoBack(webview);
//         },
//         GoForward => unsafe {
//             call_api_or_panic().mbGoForward(webview);
//         },
//         Resize(width, height) => unsafe {
//             call_api_or_panic().mbResize(webview, width, height);
//         },
//         GetWindowHandle => todo!(),
//         EditorSelectAll => unsafe {
//             call_api_or_panic().mbEditorSelectAll(webview);
//         },
//         EditorUnSelect => unsafe {
//             call_api_or_panic().mbEditorUnSelect(webview);
//         },
//         EditorCopy => unsafe {
//             call_api_or_panic().mbEditorCopy(webview);
//         },
//         EditorCut => unsafe {
//             call_api_or_panic().mbEditorCut(webview);
//         },
//         EditorDelete => unsafe {
//             call_api_or_panic().mbEditorDelete(webview);
//         },
//         EditorUndo => unsafe {
//             call_api_or_panic().mbEditorUndo(webview);
//         },
//         EditorRedo => unsafe {
//             call_api_or_panic().mbEditorRedo(webview);
//         },
//         EditorPaste => unsafe {
//             call_api_or_panic().mbEditorPaste(webview);
//         },
//         SetCookie(url, cookie) => unsafe {
//             call_api_or_panic().mbSetCookie(webview, url.as_ptr(), cookie.as_ptr());
//         },
//         PerformCookieCommand(cookie_command) => unsafe {
//             call_api_or_panic().mbPerformCookieCommand(webview, cookie_command as _);
//         },
//         ClearCookie => unsafe {
//             call_api_or_panic().mbClearCookie(webview);
//         },
//         SetCookieJarPath(path) => unsafe {
//             call_api_or_panic().mbSetCookieJarPath(webview, path.as_ptr());
//         },
//         SetCookieJarFullPath(path) => unsafe {
//             call_api_or_panic().mbSetCookieJarFullPath(webview, path.as_ptr());
//         },
//         SetLocalStorageFullPath(path) => unsafe {
//             call_api_or_panic().mbSetLocalStorageFullPath(webview, path.as_ptr());
//         },
//         FireMouseEvent(window_message, x, y, mouse_flags) => unsafe {
//             call_api_or_panic().mbFireMouseEvent(
//                 webview,
//                 window_message as _,
//                 x,
//                 y,
//                 mouse_flags.into(),
//             );
//         },
//         FireMouseWheelEvent(window_message, x, y, mouse_flags) => unsafe {
//             call_api_or_panic().mbFireMouseWheelEvent(
//                 webview,
//                 window_message as _,
//                 x,
//                 y,
//                 mouse_flags.into(),
//             );
//         },
//         FireKeyUpEvent(virtual_key_code, keyboard_flags, system_key) => unsafe {
//             call_api_or_panic().mbFireKeyUpEvent(
//                 webview,
//                 virtual_key_code as _,
//                 keyboard_flags as _,
//                 system_key as _,
//             );
//         },
//         FireKeyDownEvent(virtual_key_code, keyboard_flags, system_key) => unsafe {
//             call_api_or_panic().mbFireKeyDownEvent(
//                 webview,
//                 virtual_key_code as _,
//                 keyboard_flags as _,
//                 system_key as _,
//             );
//         },
//         FireKeyPressEvent(virtual_key_code, keyboard_flags, system_key) => unsafe {
//             call_api_or_panic().mbFireKeyPressEvent(
//                 webview,
//                 virtual_key_code as _,
//                 keyboard_flags as _,
//                 system_key as _,
//             );
//         },
//         SetFocus => unsafe {
//             call_api_or_panic().mbSetFocus(webview);
//         },
//         KillFocus => unsafe {
//             call_api_or_panic().mbKillFocus(webview);
//         },
//         RunJsSync(_, _, _) => todo!(),
//         SetZoomFactor(zoom_factor) => unsafe {
//             call_api_or_panic().mbSetZoomFactor(webview, zoom_factor);
//         },
//         GetZoomFactor => todo!(),
//         SetHandle(window_handle) => unsafe {
//             call_api_or_panic().mbSetHandle(webview, window_handle.inner);
//         },
//         SetHandleOffset(x, y) => unsafe {
//             call_api_or_panic().mbSetHandleOffset(webview, x, y);
//         },
//         SetUserAgent(user_agent) => unsafe {
//             call_api_or_panic().mbSetUserAgent(webview, user_agent.as_ptr());
//         },
//         EnableContextMenu(enable) => unsafe {
//             call_api_or_panic().mbSetContextMenuEnabled(webview, enable as _);
//         },
//         EnableCookie(enable) => unsafe {
//             call_api_or_panic().mbSetCookieEnabled(webview, enable as _);
//         },
//         EnableCspCheck(enable) => unsafe {
//             call_api_or_panic().mbSetCspCheckEnable(webview, enable as _);
//         },
//         EnableDiskCache(enable) => unsafe {
//             call_api_or_panic().mbSetDiskCacheEnabled(webview, enable as _);
//         },
//         EnableDragDrop(enable) => unsafe {
//             call_api_or_panic().mbSetDragDropEnable(webview, enable as _);
//         },
//         EnableHeadless(enable) => unsafe {
//             call_api_or_panic().mbSetHeadlessEnabled(webview, enable as _);
//         },
//         EnableMemoryCache(enable) => unsafe {
//             call_api_or_panic().mbSetMemoryCacheEnable(webview, enable as _);
//         },
//         EnableMouse(enable) => unsafe {
//             call_api_or_panic().mbSetMouseEnabled(webview, enable as _);
//         },
//         EnableNavigationToNewWindow(enable) => unsafe {
//             call_api_or_panic().mbSetNavigationToNewWindowEnable(webview, enable as _);
//         },
//         EnableNodeJs(enable) => unsafe {
//             call_api_or_panic().mbSetNodeJsEnable(webview, enable as _);
//         },
//         EnableNpapiPlugins(enable) => unsafe {
//             call_api_or_panic().mbSetNpapiPluginsEnabled(webview, enable as _);
//         },
//         EnableTouch(enable) => unsafe {
//             call_api_or_panic().mbSetTouchEnabled(webview, enable as _);
//         },
//         EnableSystemTouch(enable) => unsafe {
//             call_api_or_panic().mbSetSystemTouchEnabled(webview, enable as _);
//         },
//         EnableDrag(enable) => unsafe {
//             call_api_or_panic().mbSetDragEnable(webview, enable as _);
//         },
//         WindowShow(show) => unsafe {
//             call_api_or_panic().mbShowWindow(webview, show as _);
//         },
//         WindowResize(width, height) => unsafe {
//             call_api_or_panic().mbResize(webview, width, height);
//         },
//         WindowMove(x, y, width, height) => unsafe {
//             call_api_or_panic().mbMoveWindow(webview, x, y, width, height);
//         },
//         WindowMoveToCenter => unsafe {
//             call_api_or_panic().mbMoveToCenter(webview);
//         },
//         WindowSetTitle(cstring) => unsafe {
//             call_api_or_panic().mbSetWindowTitle(webview, cstring.as_ptr());
//         },
//     }
// }

pub(crate) fn invoke_command_sync<F, R>(handler: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send,
{
    struct Param<R> {
        sender: std::sync::mpsc::Sender<R>,
        handler: Box<dyn FnOnce() -> R + Send + 'static>,
    }

    extern "system" fn callback<R>(param: *mut c_void, _: *mut c_void) {
        let param = unsafe { Box::from_raw(param as *mut Param<R>) };
        let handler = param.handler;
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| handler()));
        let result = result.unwrap();
        param
            .sender
            .send(result)
            .expect("Failed to send result from UI thread");
    }

    let (sender, receiver) = std::sync::mpsc::channel::<R>();
    let handler = Box::new(handler);
    let param = Box::into_raw(Box::new(Param { sender, handler }));

    unsafe {
        call_api_or_panic().mbCallUiThreadSync(
            Some(callback::<R>),
            param as *mut c_void,
            std::ptr::null_mut(),
        )
    };

    receiver
        .recv()
        .expect("Failed to receive result from UI thread")
}
