use std::ffi::CStr;

pub type WinStr = CStr;
pub fn from_win_str(cstr: &CStr) -> LPCSTR {
    cstr.to_bytes_with_nul().as_ptr() as LPCSTR
}/*
fn ToOsStr(cmdarg: LPCSTR) -> &CStr {
    unsafe { CStr::from_ptr(cmdarg) }
}*/
pub fn from_win_str_or_null(oss: Option<&CStr>) -> LPCSTR {
    if let Some(p) = oss {
        from_win_str(p)
    } else {
        0 as *const i8
    }
}

type c_char = i8;
pub type CHAR = c_char;
type LPCSTR = *const CHAR;
type LPSTR = *mut CHAR;

/// data and header information. https://docs.microsoft.com/en-us/windows/win32/api/wininet/ns-wininet-internet_buffersa
#[repr(C)]
pub struct INTERNET_BUFFERSA {
   dwStructSize: super::DWORD,
   Next: *mut INTERNET_BUFFERSA,
   ///contains the headers. Can be NULL.
   lpcszHeader: LPCSTR,
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
impl INTERNET_BUFFERSA {
    pub fn new() -> INTERNET_BUFFERSA {
        INTERNET_BUFFERSA {
            dwStructSize: std::mem::size_of::<INTERNET_BUFFERSA>() as super::DWORD,
            Next: super::NULL as LPINTERNET_BUFFERSA,
            lpcszHeader: super::NULL as LPCSTR,
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
pub type LPINTERNET_BUFFERSA = *mut INTERNET_BUFFERSA;

#[link(name = ":libwininet.a")]
extern "system" {
    pub fn HttpEndRequestA(
        hRequest: super::HINTERNET,
        lpBuffersOut: LPINTERNET_BUFFERSA,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::BOOL;
    pub fn HttpOpenRequestA(
        hConnect: super::HINTERNET,
        lpszVerb: LPCSTR,
        lpszObjectName: LPCSTR,
        lpszVersion: LPCSTR,
        lpszReferrer: LPCSTR,
        lplpszAcceptTypes: *mut LPCSTR,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::HINTERNET;
    pub fn HttpSendRequestExA(
        hRequest: super::HINTERNET,
        lpBuffersIn: LPINTERNET_BUFFERSA,
        lpBuffersOut: LPINTERNET_BUFFERSA,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::BOOL;
    pub fn InternetConnectA(
        hInternet: super::HINTERNET,
        lpszServerName: LPCSTR,
        nServerPort: u16,
        lpszUserName: LPCSTR,
        lpszPassword: LPCSTR,
        dwService: super::DWORD,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::HINTERNET;
    pub fn InternetOpenA(
        lpszAgent: LPCSTR,
        dwAccessType: super::DWORD,
        lpszProxy: LPCSTR,
        lpszProxyBypass: LPCSTR,
        dwFlags: super::DWORD,
    ) -> super::HINTERNET;
    pub fn InternetSetStatusCallbackA(
        hInternet: super::HINTERNET,
        lpfnInternetCallback: super::INTERNET_STATUS_CALLBACK,
    ) -> super::INTERNET_STATUS_CALLBACK;
/*
    pub fn HttpAddRequestHeadersA(
        hRequest: super::HINTERNET,
        lpszHeaders: LPCSTR,
        dwHeadersLength: super::DWORD,
        dwModifiers: super::DWORD,
    ) -> super::BOOL;
    pub fn HttpQueryInfoA(
        hRequest: super::HINTERNET,
        dwInfoLevel: super::DWORD,
        lpBuffer: super::LPVOID,
        lpdwBufferLength: super::LPDWORD,
        lpdwIndex: super::LPDWORD,
    ) -> super::BOOL;
    pub fn HttpSendRequestA(
        hRequest: super::HINTERNET,
        lpszHeaders: LPCSTR,
        dwHeadersLength: super::DWORD,
        lpOptional: super::LPVOID,
        dwOptionalLength: super::DWORD,
    ) -> super::BOOL;
    
    pub fn InternetGetCookieA(
        lpszUrl: LPCSTR,
        lpszCookieName: LPCSTR,
        lpszCookieData: LPSTR,
        lpdwSize: super::LPDWORD,
    ) -> super::BOOL;
    pub fn InternetGetCookieExA(
        lpszUrl: LPCSTR,
        lpszCookieName: LPCSTR,
        lpszCookieData: LPSTR,
        lpdwSize: super::LPDWORD,
        dwFlags: super::DWORD,
        lpReserved: super::LPVOID,
    ) -> super::BOOL;
    pub fn InternetQueryOptionA(
        hInternet: super::HINTERNET,
        dwOption: super::DWORD,
        lpBuffer: super::LPVOID,
        lpdwBufferLength: super::LPDWORD,
    ) -> super::BOOL;
    pub fn InternetReadFileExA(
        hFile: super::HINTERNET,
        lpBuffersOut: LPINTERNET_BUFFERSA,
        dwFlags: super::DWORD,
        dwContext: super::DWORD_PTR,
    ) -> super::BOOL;
    pub fn InternetSetCookieA(
        lpszUrl: LPCSTR,
        lpszCookieName: LPCSTR,
        lpszCookieData: LPCSTR,
    ) -> super::BOOL;
    pub fn InternetSetCookieExA(
        lpszUrl: LPCSTR,
        lpszCookieName: LPCSTR,
        lpszCookieData: LPCSTR,
        dwFlags: super::DWORD,
        dwReserved: super::DWORD_PTR,
    ) -> super::DWORD;
    pub fn InternetSetOptionA(
        hInternet: super::HINTERNET,
        dwOption: super::DWORD,
        lpBuffer: super::LPVOID,
        dwBufferLength: super::DWORD,
    ) -> super::BOOL;
    pub fn InternetSetOptionExA(
        hInternet: super::HINTERNET,
        dwOption: super::DWORD,
        lpBuffer: super::LPVOID,
        dwBufferLength: super::DWORD,
        dwFlags: super::DWORD,
    ) -> super::BOOL;*/
 }