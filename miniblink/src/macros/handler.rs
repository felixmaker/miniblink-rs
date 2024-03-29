#[doc(hidden)]
#[macro_export]
macro_rules! bind_handler {
    ($trait: ident for $target: ident {
        $(
            $mbcallback: ident ($($param: ident: $ctype: ty),*) $(-> $creturn: ty)? => 
            $handler: ident ($($type: ty),*) $(-> $return: ty | $default: expr)?
        );* 
    }) => {
        #[doc=concat!("See [`", stringify!($trait), "`]")]
        pub trait $trait
        {
            $(
                #[doc=concat!("See `", stringify!($mbcallback), "`.")]
                fn $handler<F>(&self, callback: F)
                where
                    F: FnMut(&mut $target, $($type,)*) $(-> $return)? + 'static;
            )*
        }

        impl $trait for $target {
        $(
            fn $handler<F>(&self, callback: F)
            where
                F: FnMut(&mut $target, $($type,)*) $(-> $return)? + 'static,
                // $($type: FromFFI<$ctype>,)*
            {
                unsafe extern "C" fn shim<F>(
                    wv_ptr: miniblink_sys::wkeWebView,
                    c_ptr: *mut ::std::os::raw::c_void,
                    $($param: $ctype,)*
                ) $(-> $creturn)?
                where F: FnMut(&mut $target, $($type,)*) $(-> $return)? + 'static,
                {
                    let mut wv: $target = FromFFI::from(wv_ptr);
                    let cb: *mut F = c_ptr as _;
                    let f = &mut *cb;
                    $(
                        let $param: $type = FromFFI::from($param);
                    )*

                    #[allow(unused)]
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, $($param,)*)));
                    $(r.unwrap_or($default))?
                }

                let cb: *mut F = Box::into_raw(Box::new(callback));
                unsafe {
                    crate::call_api_or_panic().$mbcallback(
                        ToFFI::to(self),
                        Some(shim::<F>),
                        cb as *mut _,
                    );
                }
            }
        )*
        }
    }
}
