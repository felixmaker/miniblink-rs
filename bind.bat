@echo off

bindgen -o miniblink-sys\src\miniblink.rs mb.h --dynamic-loading Library ^
    --allowlist-function mb.* --allowlist-type mb.* ^
    --no-layout-tests --no-prepend-enum-name ^
    -- --target=i686-pc-windows-msvc
    