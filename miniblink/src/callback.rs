use crate::{net_job::NetJob, params::*, types::*, webview::WebView};

macro_rules! define_callback {
    (
        $(
            $(#[$attr:meta])*
            $name:ident: ($($arg:ty),*) $(-> $ret:ty)?
        );* $(;)?
    ) => {
        $(
            $(#[$attr])*
            pub trait $name: Fn($($arg),*) $(-> $ret)? + Send + 'static {}

            impl<T> $name for T where T: Fn($($arg),*) $(-> $ret)? + Send + 'static {}
        )*
    };
}
// Core Callbacks
define_callback!(
    /// Triggered when download happenned.
    OnDownload: (&WebView, &DownloadParameters) -> bool;
    /// Triggered when the main frame has finished loading.
    OnDocumentReady: (&WebView, &WebFrameHandle);
    /// Triggered before navigating to a new URL.
    OnNavigation: (&WebView, &NavigationParameters) -> bool;
    /// Triggered when the page requests a new window/webview.
    OnCreateView: (&WebView, &CreateViewParameters) -> Option<WebView>
);

// Logic Callbacks
define_callback!(
    /// Triggered by window.mbQuery in JavaScript, for example:
    /// 
    /// function onNative(customMsg, response) {
    ///     console.log("on~~mbQuery:" + response);
    /// }
    /// window.mbQuery(0x123456, "test run", onNative);
    OnQuery: (&WebView, &JsQueryParameters) -> JsQueryResult;
    /// Triggered when the URL changes.
    OnUrlChanged: (&WebView, &UrlChangedParameters);
    /// Triggered when the document title changes.
    OnTitleChanged: (&WebView, &str)
);

// Dialog Callbacks
define_callback!(
    /// Triggered by window.alert().
    OnAlertBox: (&WebView, &str);
    /// Triggered by window.confirm().
    OnConfirmBox: (&WebView, &str) -> bool;
    /// Triggered by window.prompt().
    OnPromptBox: (&WebView, &PromptParams) -> Option<String>
);

// Window Callbacks
define_callback!(
    /// Triggered when the window is requesting to close.
    OnClose: (&WebView) -> bool;
    /// Triggered when the window is being destroyed.
    OnDestroy: (&WebView) -> bool
);

// Network Callbacks
define_callback!(
    /// Triggered before a network request starts.
    OnLoadUrlBegin: (&WebView, &str, &NetJob) -> bool;
    /// Triggered after a network request finishes.
    OnLoadUrlEnd: (&WebView, &str, &NetJob, &[u8])
);
