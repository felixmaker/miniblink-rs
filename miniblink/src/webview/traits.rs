/// Web Frame Handle
pub type WebFrameHandle = isize;

/// Unique ID to identify the query message.
pub type QueryMessage = i32;

/// Defines the webview function switch.
pub trait WebViewSwitch {
    /// Enable context menu.
    fn enable_context_menu(&self, enabled: bool);
    /// Enable cookie.
    fn enable_cookie(&self, enabled: bool);
    /// Enable CSP check.
    fn enable_csp_check(&self, enabled: bool);
    /// Enable disk cache.
    fn enable_disk_cache(&self, enabled: bool);
    /// Enable drag and drop.
    fn enable_drag_drop(&self, enabled: bool);
    /// Enable drag.
    fn enable_drag(&self, enabled: bool);
    /// Enable headless mode.
    fn enable_headless(&self, enabled: bool);
    /// Enable memory cache.
    fn enable_memory_cache(&self, enabled: bool);
    /// Enable mouse.
    fn enable_mouse(&self, enabled: bool);
    /// Enable navigation to new window.
    fn enable_navigation_to_new_window(&self, enabled: bool);
    /// Enable nodejs.
    fn enable_nodejs(&self, enabled: bool);
    /// Enable npapi plugins.
    fn enable_npapi_plugins(&self, enabled: bool);
    /// Enable system touch.
    fn enable_system_touch(&self, enabled: bool);
    /// Enable touch.
    fn enable_touch(&self, enabled: bool);
}

/// Defines the webview operation.
pub trait WebViewOperation {
    /// Load html with base url.
    fn load_html_with_base_url(&self, html: &str, base_url: &str);
    /// Load url.
    fn load_url(&self, url: &str);

    /// Reload page.
    fn reload(&self);
    /// Stop loading the page.
    fn stop_loading(&self);
    /// Go back.
    fn go_back(&self);
    /// Go forward.
    fn go_forward(&self);

    /// Set the user agent.
    fn set_user_agent(&self, user_agent: &str);

    /// Check if is the mainframe.
    fn is_mainframe(&self, frame_handle: WebFrameHandle) -> bool;
}

/// Defines the web window operation.
pub trait WebWindowOperation {
    /// Show the window.
    fn show(&self);
    /// Hide the window.
    fn hide(&self);

    /// Resize the window.
    fn resize(&self, w: i32, h: i32);
    /// Set window focus.
    fn set_focus(&self);
    /// Kill window focuse.
    fn kill_focus(&self);

    /// Move the window.
    fn move_window(&self, x: i32, y: i32, w: i32, h: i32);
    /// Move the window to center.
    fn move_to_center(&self);
    /// Set the window title.
    fn set_window_title(&self, title: &str);
}

/// Defines the editor operation.
pub trait WebViewEditorOperation {
    /// Send copy command to editor.
    fn editor_copy(&self);
    /// Send cut command to editor.
    fn editor_cut(&self);
    /// Send delete command to editor.
    fn editor_delete(&self);
    /// Send paste command to editor.
    fn editor_paste(&self);
    /// Send redo command to editor.
    fn editor_redo(&self);
    /// Send select command to editor.
    fn editor_select_all(&self);
    /// Send unselect command to editor.
    fn editor_unselect(&self);
    /// Send undo command to editor.
    fn editor_undo(&self);
}

/// Defines the webview event.
pub trait WebViewEvent {
    /// Call on title changed.
    ///
    /// - param1: String, document title
    fn on_title_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut Self, String) + 'static;
    /// Call on url changed.
    ///
    /// - param1: String, url
    /// - param2: bool, if can go back
    /// - param3: bool, if can go forward
    fn on_url_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut Self, String, bool, bool) + 'static;
    /// Call on document ready.
    ///
    /// - param1: bool, if is the main frame
    fn on_document_ready<F>(&self, callback: F)
    where
        F: FnMut(&mut Self, WebFrameHandle) + 'static;
    /// Call on the window close.
    fn on_close<F>(&self, callback: F)
    where
        F: FnMut(&mut Self) -> bool + 'static;
    /// Call on the window destroy.
    fn on_destroy<F>(&self, callback: F)
    where
        F: FnMut(&mut Self) -> bool + 'static;
}

/// Defines Javascipt call.
pub trait WebViewJsCall {
    /// Eval a script on the frame.
    fn eval(&self, frame_id: isize, script: &str, is_in_closure: bool) -> String;

    /// On js query.
    fn on_query<F>(&self, callback: F)
    where
        F: FnMut(&mut Self, QueryMessage, String) -> (QueryMessage, String) + 'static;
}
