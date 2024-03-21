@echo off

patch -o miniblink-sys\header\wke.h miniblink-sys\header\wke.h.origin miniblink-sys\header\wke.patch
bindgen -o miniblink-sys\src\miniblink.rs miniblink-sys\header\wrapper.h --dynamic-loading Library --allowlist-function wke.* --default-enum-style=newtype --no-layout-tests --allowlist-function js.*
