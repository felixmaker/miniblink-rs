// #[doc(hidden)]
// #[macro_export]
// macro_rules! impl_target {
//     ($trait: ident for $target: ident {
//         $(
//             $mbcallback: ident => $func: ident ($($param: ident: $type: ty $(as $cross_type: ty)?),*) $(-> $return: ty)?
//         );*$(;)?
//     }) => {
//         #[allow(unused)]
//         #[doc=concat!("See [`", stringify!($trait), "`]")]
//         pub trait $trait {
//             $(
//                 #[doc=concat!("See `", stringify!($mbcallback), "`.")]
//                 fn $func(&self, $($param: $type,)*) $(-> $return)?;
//             )*
//         }

//         impl $trait for $target {
//             $(
//                 fn $func(&self, $($param: $type,)*) $(-> $return)? {
//                     use crate::types::*;
//                     $(
//                         $(let $param: $cross_type = $param.prepare();)?
//                         let $param = $param.to();
//                     )*
//                     #[allow(unused)]
//                     let r = unsafe {
//                         crate::call_api_or_panic().$mbcallback(self.to(), $($param,)*)
//                     };
//                     $(
//                         let r: $return = FromFFI::from(r);
//                         r
//                     )?
//                 }
//             )*
//         }
//     }
// }

#[doc(hidden)]
#[macro_export]
macro_rules! bind_target {
    ($(
        $vis: vis $mbcallback: ident => $func: ident ($($param: ident: $type: ty $(as $cross_type: ty)?),*) $(-> $return: ty)?
    );*$(;)?) => {
        $(
            #[doc=concat!("See `", stringify!($mbcallback), "`")]
            $vis fn $func(&self, $($param: $type,)*) $(-> $return)? {
                use crate::ffi::*;
                $(
                    $(let $param: $cross_type = $param.prepare();)?
                    let $param = ToFFI::to(&$param);
                )*
                #[allow(unused)]
                let r = unsafe {
                    crate::call_api_or_panic().$mbcallback(ToFFI::to(self), $($param,)*)
                };
                $(
                    let r: $return = FromFFI::from(r);
                    r
                )?
            }
        )*
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bind_target_global {
    ($(
        $vis: vis $mbcallback: ident => $func: ident ($($param: ident: $type: ty $(as $cross_type: ty)?),*) $(-> $return: ty)?
    );*$(;)?) => {
        $(
            #[doc=concat!("See `", stringify!($mbcallback), "`")]
            $vis fn $func(&self, $($param: $type,)*) $(-> $return)? {
                use crate::ffi::*;
                $(
                    $(let $param: $cross_type = $param.prepare();)?
                    let $param = ToFFI::to(&$param);
                )*
                #[allow(unused)]
                let r = unsafe {
                    call_api_or_panic().$mbcallback($($param,)*)
                };
                $(
                    let r: $return = FromFFI::from(r);
                    r
                )?
            }
        )*
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bind_global {
    ($(
        $vis: vis $mbcallback: ident => $func: ident ($($param: ident: $type: ty $(as $cross_type: ty)?),*) $(-> $return: ty)?
    );*$(;)?) => {
        $(
            #[doc=concat!("See `", stringify!($mbcallback), "`")]
            $vis fn $func($($param: $type,)*) $(-> $return)? {
                #[allow(unused)]
                use crate::ffi::*;
                $(
                    $(let $param: $cross_type = $param.prepare();)?
                    let $param = $param.to();
                )*
                #[allow(unused)]
                let r = unsafe {
                    crate::call_api_or_panic().$mbcallback($($param,)*)
                };
                $(
                    let r: $return = FromFFI::from(r);
                    r
                )?
            }
        )*
    }
}
