use std::ffi::CString;

use crate::types::*;
use miniblink_sys::*;

use crate::util::SafeCString;

pub trait PrepareFFI<T> {
    fn prepare(&self) -> T;
}

impl PrepareFFI<CString> for &str {
    fn prepare(&self) -> CString {
        CString::safe_new(&self)
    }
}

impl PrepareFFI<WkeString> for &str {
    fn prepare(&self) -> WkeString {
        WkeString::new(&self)
    }
}

impl PrepareFFI<CProxy> for Proxy {
    fn prepare(&self) -> CProxy {
        CProxy::new(&self)
    }
}

impl PrepareFFI<wkeViewSettings> for ViewSettings {
    fn prepare(&self) -> wkeViewSettings {
        wkeViewSettings {
            size: self.size,
            bgColor: self.backgroud_color,
        }
    }
}

impl PrepareFFI<POINT> for Point {
    fn prepare(&self) -> POINT {
        POINT {
            x: self.x,
            y: self.y,
        }
    }
}
