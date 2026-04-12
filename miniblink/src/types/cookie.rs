
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The cookie command.
pub enum CookieCommand {
    /// Clear all cookies. Same as curl command: CURLOPT_COOKIELIST, "ALL".
    ClearAllCookies = 0,
    /// Clear session cookies. Same as curl command: CURLOPT_COOKIELIST, "SESS"
    ClearSessionCookies = 1,
    /// Flush cookies to file. Same as curl command: CURLOPT_COOKIELIST, "FLUSH".
    FlushCookiesToFile = 2,
    /// Reload cookies from file. Same as curl command: CURLOPT_COOKIELIST, "RELOAD".
    ReloadCookiesFromFile = 3,
}
