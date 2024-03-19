mod handler;
mod util;

pub mod app;
pub mod error;
pub mod proxy;
pub mod value;
pub mod webview;
pub mod wstr;

#[warn(missing_docs)]
use std::sync::Arc;

use error::Result;

lazy_static::lazy_static! {
    pub static ref API: Arc<miniblink_sys::api> =Arc::new(load_miniblink_api().unwrap());
}

fn load_miniblink_api() -> Result<miniblink_sys::api> {
    let lib: miniblink_sys::api = unsafe { miniblink_sys::api::new("node.dll")? };
    unsafe { lib.wkeInitialize() };
    Ok(lib)
}
