
type wchar_t = u16;
type WCHAR = wchar_t;
type LPCWSTR = *const WCHAR;
type PCWSTR = *const WCHAR;
type LPWSTR = *mut WCHAR;
type PWSTR = *mut WCHAR;

use widestring::{U16CStr, U16CString};
use std::slice;
pub type WinStr = U16CStr;

pub fn FromWinStr(oss: &U16CStr) -> LPCWSTR {
    oss.as_ptr()
}/*
fn ToOsStr(cmdarg: LPCWSTR) -> U16CString {
    unsafe {U16CString::from_ptr_str(cmdarg)}
}*/

pub fn FromWinStrOrNull(oss: Option<&U16CStr>) -> LPCWSTR {
    if let Some(p) = oss {
        FromWinStr(p)
    } else {
        0 as *const u16
    }
}

/// data and header information. https://docs.microsoft.com/en-us/windows/win32/api/wininet/ns-wininet-internet_buffersw
#[repr(C)]
pub struct INTERNET_BUFFERSW {
    dwStructSize: super::DWORD,
    Next: *mut INTERNET_BUFFERSW,
    ///contains the headers. Can be NULL.
    lpcszHeader: LPCWSTR,
    ///Size of the headers, in TCHARs, if lpcszHeader is not NULL
    dwHeadersLength: super::DWORD,
    ///Size of the headers, if there is not enough memory in the buffer.
    dwHeadersTotal: super::DWORD,
    ///data buffer (body)
    lpvBuffer: super::LPVOID,
    ///Size of the buffer, in bytes, if lpvBuffer is not NULL
    dwBufferLength: super::DWORD,
    /// content-length
    dwBufferTotal: super::DWORD,
    ///Reserved
    dwOffsetLow: super::DWORD,
    ///Reserved
    dwOffsetHigh: super::DWORD,
}
impl INTERNET_BUFFERSW {
    pub fn new() -> INTERNET_BUFFERSW {
        INTERNET_BUFFERSW {
            dwStructSize: std::mem::size_of::<INTERNET_BUFFERSW>() as super::DWORD,
            Next: super::NULL as LPINTERNET_BUFFERSW,
            lpcszHeader: super::NULL as LPCWSTR,
            dwHeadersLength: 0,
            dwHeadersTotal: 0,
            lpvBuffer: super::NULL,
            dwBufferLength: 0,
            dwBufferTotal: 0,
            dwOffsetLow: 0,
            dwOffsetHigh: 0,
        }
    }
    pub fn set_content_len(&mut self, len: super::DWORD) {
        self.dwBufferTotal = len;
    }
}
type LPINTERNET_BUFFERSW = *mut INTERNET_BUFFERSW;

#[repr(C)]
pub struct INTERNET_COOKIE2 {
    pwszName: PWSTR,
    pwszValue: PWSTR,
    pwszDomain: PWSTR,
    pwszPath: PWSTR,
    dwFlags: super::DWORD,
    ftExpires: super::FILETIME,
    fExpiresSet: super::BOOL,
}

#[link(name = ":libwininet.a")]
extern "system" {
    pub fn HttpAddRequestHeadersW(
        hRequest: super::HINTERNET,
        lpszHeaders: LPCWSTR,
        dwHeadersLength: super::DWORD,
        dwModifiers: super::DWORD,
    ) -> super::BOOL;
    pub fn HttpEndRequestW(
        hRequest: super::HINTERNET,
        lpBuffersOut: LPINTERNET_BUFFERSW,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::BOOL;
    pub fn HttpOpenRequestW(
        hConnect: super::HINTERNET,
        lpszVerb: LPCWSTR,
        lpszObjectName: LPCWSTR,
        lpszVersion: LPCWSTR,
        lpszReferrer: LPCWSTR,
        lplpszAcceptTypes: *mut LPCWSTR,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::HINTERNET;
    pub fn HttpQueryInfoW(
        hRequest: super::HINTERNET,
        dwInfoLevel: super::DWORD,
        lpBuffer: super::LPVOID,
        lpdwBufferLength: super::LPDWORD,
        lpdwIndex: super::LPDWORD,
    ) -> super::BOOL;
    pub fn HttpSendRequestExW(
        hRequest: super::HINTERNET,
        lpBuffersIn: LPINTERNET_BUFFERSW,
        lpBuffersOut: LPINTERNET_BUFFERSW,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::BOOL;
    pub fn HttpSendRequestW(
        hRequest: super::HINTERNET,
        lpszHeaders: LPCWSTR,
        dwHeadersLength: super::DWORD,
        lpOptional: super::LPVOID,
        dwOptionalLength: super::DWORD,
    ) -> super::BOOL;
    pub fn InternetConnectW(
        hInternet: super::HINTERNET,
        lpszServerName: LPCWSTR,
        nServerPort: u16,
        lpszUserName: LPCWSTR,
        lpszPassword: LPCWSTR,
        dwService: super::DWORD,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::HINTERNET;

    pub fn InternetGetCookieEx2(
        pcwszUrl: PCWSTR,
        pcwszCookieName: PCWSTR,
        dwFlags: super::DWORD,
        ppCookies: *mut *mut INTERNET_COOKIE2,
        pdwCookieCount: super::PDWORD,
    ) -> super::DWORD;
    pub fn InternetGetCookieExW(
        lpszUrl: LPCWSTR,
        lpszCookieName: LPCWSTR,
        lpszCookieData: LPWSTR,
        lpdwSize: super::LPDWORD,
        dwFlags: super::DWORD,
        lpReserved: super::LPVOID,
    ) -> super::BOOL;
    pub fn InternetGetCookieW(
        lpszUrl: LPCWSTR,
        lpszCookieName: LPCWSTR,
        lpszCookieData: LPWSTR,
        lpdwSize: super::LPDWORD,
    ) -> super::BOOL;
    pub fn InternetOpenW(
        lpszAgent: LPCWSTR,
        dwAccessType: super::DWORD,
        lpszProxy: LPCWSTR,
        lpszProxyBypass: LPCWSTR,
        dwFlags: super::DWORD,
    ) -> super::HINTERNET;
    pub fn InternetQueryOptionW(
        hInternet: super::HINTERNET,
        dwOption: super::DWORD,
        lpBuffer: super::LPVOID,
        lpdwBufferLength: super::LPDWORD,
    ) -> super::BOOL;
    pub fn InternetReadFileExW(
        hFile: super::HINTERNET,
        lpBuffersOut: LPINTERNET_BUFFERSW,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::BOOL;
    pub fn InternetSetCookieEx2(
        pcwszUrl: PCWSTR,
        pCookie: *const INTERNET_COOKIE2,
        pcwszP3PPolicy: PCWSTR,
        dwFlags: super::DWORD,
        pdwCookieState: super::PDWORD,
    ) -> super::DWORD;
    pub fn InternetSetCookieExW(
        lpszUrl: LPCWSTR,
        lpszCookieName: LPCWSTR,
        lpszCookieData: LPCWSTR,
        dwFlags: super::DWORD,
        dwReserved: super::DWORD_PTR,
    ) -> super::DWORD;
    pub fn InternetSetCookieW(
        lpszUrl: LPCWSTR,
        lpszCookieName: LPCWSTR,
        lpszCookieData: LPCWSTR,
    ) -> super::BOOL;
    pub fn InternetSetOptionExW(
        hInternet: super::HINTERNET,
        dwOption: super::DWORD,
        lpBuffer: super::LPVOID,
        dwBufferLength: super::DWORD,
        dwFlags: super::DWORD,
    ) -> super::BOOL;
    pub fn InternetSetOptionW(
        hInternet: super::HINTERNET,
        dwOption: super::DWORD,
        lpBuffer: super::LPVOID,
        dwBufferLength: super::DWORD,
    ) -> super::BOOL;
    pub fn InternetSetStatusCallbackW(
        hInternet: super::HINTERNET,
        lpfnInternetCallback: super::INTERNET_STATUS_CALLBACK,
    ) -> super::INTERNET_STATUS_CALLBACK;
}
