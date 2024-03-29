#[doc(hidden)]
#[macro_export]
macro_rules! bind_props_get {
    ($trait: ident for $target: ident { $($mbcallback: ident => $prop: ident: $return: ty);* }) => {
        #[doc=concat!("Handler for Webview. See [`", stringify!($trait), "`]")]
        pub trait $trait {
            $(
                paste::paste! {
                    #[doc=concat!("Get the ", stringify!($prop), " of [`", stringify!($target) , "`]. See ", stringify!($mbcallback), ".")]
                    fn [<get_ $prop>](&self) -> $return;
                }
            )*
        }

        impl $trait for $target {
            $(
                paste::paste! {
                    fn [<get_ $prop>](&self) -> $return {
                        let r = unsafe {
                            call_api_or_panic().$mbcallback(self.webview)
                        };
                        FromFFI::from(r)
                    }
                }
            )*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bind_props_set {
    ($trait: ident for $target: ident { $($mbcallback: ident => $prop: ident: $prop_type: ty);* }) => {
        #[doc=concat!("Handler for Webview. See [`", stringify!($trait), "`]")]
        pub trait $trait {
            $(
                paste::paste! {
                    #[doc=concat!("Get the ", stringify!($handler), " of [`", stringify!($target) , "`]. See ", stringify!($mbcallback), ".")]
                    fn [<set_ $prop>](&self, $prop: $prop_type);
                }
            )*
        }

        impl $trait for $target {
            $(
                paste::paste! {
                    fn [<set_ $prop>](&self, $prop: $prop_type) {
                        unsafe {
                            call_api_or_panic().$mbcallback(self.webview, ToFFI::to(&$prop))
                        }
                    }
                }
            )*
        }
    }
}
