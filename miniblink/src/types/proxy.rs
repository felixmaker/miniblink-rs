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

#[allow(missing_docs)]
/// see `wkeProxy`.
pub struct Proxy {
    pub type_: ProxyType,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Proxy {
    pub(crate) fn to_wke_proxy(&self) -> wkeProxy {
        wkeProxy {
            type_: self.type_.to_wke_proxy_type(),
            hostname: string_to_slice(&self.hostname),
            port: self.port,
            username: string_to_slice(&self.username),
            password: string_to_slice(&self.password),
        }
    }
}
