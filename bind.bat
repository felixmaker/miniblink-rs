@echo off

@REM patch -o miniblink-sys\header\wke.h miniblink-sys\header\wke.h.origin miniblink-sys\header\wke.patch
bindgen -o miniblink-sys\src\miniblink.rs miniblink-sys\header\wrapper.h --dynamic-loading Library ^
    --allowlist-function wke.* --allowlist-function js.* --allowlist-type wke.* ^
    --blocklist-type "BOOL|HANDLE|WPARAM|LPARAM|LRESULT|HWND|HWND__|HDC__|HDC|tagRECT|RECT|tagPOINT|POINT|COLORREF|_STARTUPINFOW|STARTUPINFOW" ^
    --raw-line "use super::header::*;" ^
    --default-enum-style=newtype --no-layout-tests

