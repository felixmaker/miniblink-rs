# miniblink-sys

Rust raw bindings to miniblink49

bindgen -o miniblink-sys\src\miniblink.rs miniblink-sys\header\wrapper.h --dynamic-loading Library --allowlist-function wke.* --default-enum-style=newtype --no-layout-tests --allowlist-function js.*
