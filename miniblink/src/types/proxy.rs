/// The proxy type.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ProxyType {
    /// Http proxy.
    Http = 0,
    /// Https proxy.
    Https = 1,
    /// Socks4 proxy.
    Socks4 = 2,
    /// Socks4A proxy.
    Socks4A = 3,
    /// Socks5 proxy.
    Socks5 = 4,
    /// Socks5 hostname proxy.
    SocksHostname = 5,
}

/// The proxy information.
pub struct Proxy {
    /// The proxy type.
    pub type_: ProxyType,
    /// The proxy hostname. less than 100 characters.
    pub hostname: String,
    /// The proxy port.
    pub port: u16,
    /// The proxy username. less than 50 characters.
    pub username: String,
    /// The proxy password. less than 50 characters.
    pub password: String,
}

impl Proxy {
    pub(crate) fn to_mb_proxy(&self) -> miniblink_sys::mbProxy {
        use std::cmp::min;
        use std::ffi::c_char;
        
        fn copy_to_char_array(src: &str, dst: &mut [c_char]) {
            let bytes = src.as_bytes();
            let len = min(bytes.len(), dst.len());
            for i in 0..len {
                dst[i] = bytes[i] as std::ffi::c_char;
            }
        }
        
        let mut mb_proxy = miniblink_sys::mbProxy {
            type_: self.type_ as _,
            hostname: [0; 100],
            port: self.port,
            username: [0; 50],
            password: [0; 50],
        };

        copy_to_char_array(&self.hostname, &mut mb_proxy.hostname);
        copy_to_char_array(&self.username, &mut mb_proxy.username);
        copy_to_char_array(&self.password, &mut mb_proxy.password);

        mb_proxy
    }
}