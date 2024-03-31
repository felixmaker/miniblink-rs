#[doc(hidden)]
#[macro_export]
macro_rules! count_one {
    ($any: ident) => {
        1
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! js_bind_function_ext {
    ($(
        $vis: vis $func: ident ($($param: ident),*)
    );*) => {
        $(
            #[doc=concat!("Js bind function, with params `", $(stringify!($param),)* "`")]
            $vis fn $func<$($param,)* T, F>(name: &str, func: F)
            where
                F: Fn($($param,)*) -> MBResult<T> + 'static,
                JsExecState: $(MBExecStateValue<$param> +)* MBExecStateValue<T>,
            {
                #[allow(unused)]
                use crate::types::JsExecStateExt;
                js_bind_function(
                    name,
                    move |es| {
                        $(
                            #[allow(non_snake_case)]
                            let $param = es.get_arg_value(0).unwrap();
                        )*
                        es.js_value(func($($param,)*)?)
                    },
                    0 $(+ crate::count_one!($param))*,
                );
            }
        )*
    }
}
