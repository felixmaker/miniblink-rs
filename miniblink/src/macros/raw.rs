#[doc(hidden)]
#[macro_export]
macro_rules! bind_target {
    ($trait: ident for $target: ident {
        $(
            $mbcallback: ident => $func: ident ($($param: ident: $type: ty),*) $(-> $return: ty)?
        );*
    }) => {
        #[allow(unused)]
        #[doc=concat!("See [`", stringify!($trait), "`]")]
        pub trait $trait {
            $(
                #[doc=concat!("See `", stringify!($mbcallback), "`.")]
                fn $func(&self, $($param: $type,)*) $(-> $return)?;
            )*
        }

        impl $trait for $target {
            $(
                fn $func(&self, $($param: $type,)*) $(-> $return)? {
                    $(
                        let $param = ToFFI::to(&$param);
                    )*
                    #[allow(unused)]
                    let r = unsafe {
                        call_api_or_panic().$mbcallback(ToFFI::to(self), $($param,)*)
                    };
                    $(
                        let r: $return = FromFFI::from(r);
                        r
                    )?
                }
            )*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bind_global {
    ($(
        $vis: vis $mbcallback: ident => $func: ident ($($param: ident: $type: ty),*) $(-> $return: ty)?
    );*) => {
        $(
            #[doc=concat!("See `", stringify!($mbcallback), "`.")]
            $vis fn $func($($param: $type,)*) $(-> $return)? {
                $(
                    let $param = crate::types::ToFFI::to(&$param);
                )*
                #[allow(unused)]
                let r = unsafe {
                    call_api_or_panic().$mbcallback($($param,)*)
                };
                $(
                    let r: $return = crate::types::FromFFI::from(r);
                    r
                )?
            }
        )*
    }
}
