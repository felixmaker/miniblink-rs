use std::ffi::CString;

use miniblink_sys::{wkeProxy, wkeProxyType};

use crate::util::SafeCString;

/// Proxy Endpoint
#[derive(Debug, Clone)]
pub struct ProxyEndpoint {
    /// Proxy server host (e.g. 192.168.0.100, localhost, example.com, etc.)
    pub host: String,
    /// Proxy server port (e.g. 1080, 3128, etc.)
    pub port: u16,
    /// Proxy server username
    pub username: Option<String>,
    /// Proxy server password
    pub password: Option<String>,
}

/// Proxy Config. Support HTTP, SOCKS4, SOCKS4A, SOCKS5, SOCKS5HOSTNAME
#[derive(Debug, Clone)]
pub enum ProxyConfig {
    /// Connect to no proxy server
    None,
    /// Connect to proxy server via HTTP CONNECT
    Http(ProxyEndpoint),
    /// Connect to proxy server via SOCKSv4
    Socks4(ProxyEndpoint),
    /// Connect to proxy server via SOCKSv4A
    Socks4A(ProxyEndpoint),
    /// Connect to proxy server via SOCKSv5
    Socks5(ProxyEndpoint),
    /// Connect to proxy server via SOCKSv5 Hostname
    Socks5Hostname(ProxyEndpoint),
}

impl ProxyConfig {
    pub(crate) fn to_wke_proxy(&self) -> wkeProxy {
        match self {
            Self::None => wkeProxy {
                type_: wkeProxyType::WKE_PROXY_NONE,
                hostname: [0; 100],
                port: 0,
                username: [0; 50],
                password: [0; 50],
            },
            Self::Http(endpoint) => wkeproxy_new(
                wkeProxyType::WKE_PROXY_HTTP,
                endpoint.host.as_str(),
                endpoint.port,
                endpoint.username.as_deref(),
                endpoint.password.as_deref(),
            ),
            Self::Socks4(endpoint) => wkeproxy_new(
                wkeProxyType::WKE_PROXY_SOCKS4,
                endpoint.host.as_str(),
                endpoint.port,
                endpoint.username.as_deref(),
                endpoint.password.as_deref(),
            ),
            Self::Socks4A(endpoint) => wkeproxy_new(
                wkeProxyType::WKE_PROXY_SOCKS4A,
                endpoint.host.as_str(),
                endpoint.port,
                endpoint.username.as_deref(),
                endpoint.password.as_deref(),
            ),
            Self::Socks5(endpoint) => wkeproxy_new(
                wkeProxyType::WKE_PROXY_SOCKS5,
                endpoint.host.as_str(),
                endpoint.port,
                endpoint.username.as_deref(),
                endpoint.password.as_deref(),
            ),
            Self::Socks5Hostname(endpoint) => wkeproxy_new(
                wkeProxyType::WKE_PROXY_SOCKS5HOSTNAME,
                endpoint.host.as_str(),
                endpoint.port,
                endpoint.username.as_deref(),
                endpoint.password.as_deref(),
            ),
        }
    }
}

fn string_to_slice<const N: usize>(s: &str) -> [i8; N] {
    let mut vec = vec![0; N];
    let bytes = CString::safe_new(s).into_bytes_with_nul();
    for i in 0..bytes.len() {
        if i < N {
            vec[i] = bytes[i] as i8
        }
    }
    vec.try_into().unwrap()
}

fn wkeproxy_new(
    type_: wkeProxyType,
    hostname: &str,
    port: u16,
    username: Option<&str>,
    password: Option<&str>,
) -> wkeProxy {
    wkeProxy {
        type_,
        hostname: string_to_slice(hostname),
        port: port as u16,
        username: string_to_slice(username.unwrap_or("")),
        password: string_to_slice(password.unwrap_or("")),
    }
}
