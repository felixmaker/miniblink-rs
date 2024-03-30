use miniblink_sys::{wkeProxy, wkeProxyType};

use crate::util::string_to_slice;

#[allow(missing_docs)]
/// Proxy type
pub enum ProxyType {
    None,
    Http,
    Socks4,
    Socks4A,
    Socks5,
    Socks5Hostname,
}

/// A proxy config struct
pub struct Proxy {
    type_: ProxyType,
    hostname: String,
    port: u16,
    username: String,
    password: String,
}

#[derive(Clone, Copy)]
pub(crate) struct CProxy {
    inner: wkeProxy,
}

impl ProxyType {
    fn to_wke_proxy_type(&self) -> wkeProxyType {
        match self {
            ProxyType::None => wkeProxyType::WKE_PROXY_NONE,
            ProxyType::Http => wkeProxyType::WKE_PROXY_HTTP,
            ProxyType::Socks4 => wkeProxyType::WKE_PROXY_SOCKS4,
            ProxyType::Socks4A => wkeProxyType::WKE_PROXY_SOCKS4A,
            ProxyType::Socks5 => wkeProxyType::WKE_PROXY_SOCKS5,
            ProxyType::Socks5Hostname => wkeProxyType::WKE_PROXY_SOCKS5HOSTNAME,
        }
    }
}

impl CProxy {
    pub(crate) fn new(proxy: &Proxy) -> Self {
        let inner = wkeProxy {
            type_: proxy.type_.to_wke_proxy_type(),
            hostname: string_to_slice(&proxy.hostname),
            port: proxy.port,
            username: string_to_slice(&proxy.username),
            password: string_to_slice(&proxy.password),
        };

        Self { inner }
    }

    pub(crate) fn as_ptr(&self) -> *const wkeProxy {
        &self.inner
    }

    pub(crate) fn into_wke_proxy(self) -> wkeProxy {
        self.inner
    }
}
