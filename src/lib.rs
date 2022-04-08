use std::os::raw::c_void;
type c_int = i32;
type DWORD = u32;
type BOOL = c_int;
type c_long = i32;
const FALSE: BOOL = 0;
type PDWORD = *mut DWORD;
type LPDWORD = *mut DWORD;
pub type LPVOID = *mut c_void;
type LPCVOID = *const c_void;
type LONG = c_long;
type WORD = c_ushort;

type c_ushort = u16;


type __int8 = i8;
type __uint8 = u8;
type __int16 = i16;
type __uint16 = u16;
type __int32 = i32;
type __uint32 = u32;
type __int64 = i64;
type __uint64 = u64;

type DWORD_PTR = usize;
type PLONG = *mut LONG;

/// part of the callback
pub type HINTERNET = LPVOID;
type LPHINTERNET = *mut HINTERNET;

#[allow(non_camel_case_types)]
#[cfg(feature = "unicode")]
mod unicode;
#[cfg(feature = "unicode")]
use unicode::{
    from_win_str, from_win_str_or_null, HttpEndRequestW as HttpEndRequest,
    HttpSendRequestExW as HttpSendRequestEx, InternetConnectW as InternetConnect,
    InternetOpenW as InternetOpen, InternetSetStatusCallbackW as InternetSetStatusCallback,
    HttpOpenRequestW as HttpOpenRequest, INTERNET_BUFFERSW as INTERNET_BUFFERS
};
#[cfg(feature = "unicode")]
pub use unicode::WinStr;

#[allow(non_camel_case_types)]
#[cfg(not(feature = "unicode"))]
mod ansi;
#[cfg(not(feature = "unicode"))]
use ansi::{
    from_win_str, from_win_str_or_null, HttpEndRequestA as HttpEndRequest,
    HttpSendRequestExA as HttpSendRequestEx, InternetConnectA as InternetConnect,
    InternetOpenA as InternetOpen, InternetSetStatusCallbackA as InternetSetStatusCallback,
    HttpOpenRequestA as HttpOpenRequest, INTERNET_BUFFERSA as INTERNET_BUFFERS
};
#[cfg(not(feature = "unicode"))]
pub use ansi::WinStr;

/// part of the callback
#[repr(u32)] //DWORD
pub enum InternetStatus {
    ResolvingName = 10,
    NameResolved = 11,
    ConnectingToServer = 20,
    ConnectedToServer = 21,
    SendingRequest = 30,
    RequestSent = 31,
    ReceivingResponse = 40,
    ResponseReceived = 41,
    CtlResponseReceived = 42,
    Prefetch = 43,
    ClosingConnection = 50,
    ConnectionClosed = 51,
    HandleCreated = 60,
    HandleClosing = 70,
    DetectingProxy = 80,
    RequestComplete = 100,
    Redirect = 110,
    IntermediateResponse = 120,
    UserInputRequired = 140,
    StateChange = 200,
    CookieSent = 320,
    CookieReceived = 321,
    PrivacyImpacted = 324,
    P3pHeader = 325,
    P3pPolicyref = 326,
    CookieHistory = 327,
}

/// part of the callback
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct INTERNET_ASYNC_RESULT {
    pub dwResult: DWORD_PTR,
    pub dwError: DWORD,
}

#[repr(C)]
struct FILETIME {
    dwLowDateTime: DWORD,
    dwHighDateTime: DWORD,
}

pub type INTERNET_STATUS_CALLBACK = Option<
    unsafe extern "system" fn(
        hInternet: HINTERNET,
        dwContext: DWORD_PTR,
        dwInternetStatus: DWORD,
        lpvStatusInformation: LPVOID,
        dwStatusInformationLength: DWORD,
    ),
>;

pub type LPINTERNET_STATUS_CALLBACK = *mut INTERNET_STATUS_CALLBACK;

#[allow(non_camel_case_types)]
#[link(name = ":libwininet.a")]
extern "system" {
    fn InternetQueryDataAvailable(
        hFile: HINTERNET,
        lpdwNumberOfBytesAvailable: LPDWORD,
        dwFlags: DWORD,
        dwContext: DWORD_PTR,
    ) -> BOOL;
    fn InternetReadFile(
        hFile: HINTERNET,
        lpBuffer: LPVOID,
        dwNumberOfBytesToRead: DWORD,
        lpdwNumberOfBytesRead: LPDWORD,
    ) -> BOOL;
    fn InternetSetFilePointer(
        hFile: HINTERNET,
        lDistanceToMove: LONG,
        lpDistanceToMoveHigh: PLONG,
        dwMoveMethod: DWORD,
        dwContext: DWORD_PTR,
    ) -> DWORD;
    fn InternetWriteFile(
        hFile: HINTERNET,
        lpBuffer: LPCVOID,
        dwNumberOfBytesToWrite: DWORD,
        lpdwNumberOfBytesWritten: LPDWORD,
    ) -> BOOL;

    fn InternetCloseHandle(hInternet: HINTERNET) -> BOOL;
}

#[repr(u32)] //DWORD
pub enum InternetOpenType {
    ///Retrieves the proxy or direct configuration from the registry.
    Preconfig = 0,
    ///Resolves all host names locally.
    Direct = 1,
    ///Use Proxy
    Proxy = 3,
    ///PRECONFIG without INS
    PreconfigWithNoAutoproxy = 4,
}
const INTERNET_FLAG_ASYNC: DWORD = 0x10000000;
const INTERNET_INVALID_STATUS_CALLBACK: isize = -1;
const NULL: *mut c_void = 0 as *mut c_void;
use std::io::Error;

/// A WinINet "session"
/// 
/// Create it with `WinINet::new`
pub struct WinINet(HINTERNET);
impl WinINet {
    pub fn new(
        agent: &WinStr,
        open_type: InternetOpenType,
        proxy: Option<&WinStr>,
        direct_hosts: Option<&WinStr>,
    ) -> Result<WinINet, Error> {

        let h = unsafe { //"session handle"
            InternetOpen(
                from_win_str(agent),
                open_type as DWORD,
                from_win_str_or_null(proxy),
                from_win_str_or_null(direct_hosts),
                INTERNET_FLAG_ASYNC,
            )
        };
        if h == NULL {
            return Err(Error::last_os_error());
        }
        Ok(WinINet(h))
    }
    pub fn set_callback(&mut self, cb: INTERNET_STATUS_CALLBACK) -> Result<(), ()> {
        let r = unsafe { InternetSetStatusCallback(self.0, cb) };
        if unsafe { std::mem::transmute::<INTERNET_STATUS_CALLBACK, isize>(r) }
            == INTERNET_INVALID_STATUS_CALLBACK
        {
            Err(())
        } else {
            Ok(())
        }
    }
    pub fn connect(
        &self,
        host: &WinStr,
        port: u16,
        user: Option<&WinStr>,
        password: Option<&WinStr>,
    ) -> Result<WinINetConnection, Error> {
        let mut con = WinINetConnection(self, NULL);

        let session = unsafe {
            InternetConnect(
                self.0,
                from_win_str(host),
                port,
                from_win_str_or_null(user),
                from_win_str_or_null(password),
                INTERNET_SERVICE_HTTP,
                0,
                &con as *const WinINetConnection as DWORD_PTR,
            )
        };
        if session == NULL {
            return Err(Error::last_os_error());
        }
        println!("WinINetConnection {:?} {:?}",&con as *const WinINetConnection, session);
        con.1 = session; //"ConnectHandle"
        Ok(con)
    }
}
impl Drop for WinINet {
    fn drop(&mut self) {
        unsafe {
            InternetCloseHandle(self.0);
        }
    }
}
const INTERNET_SERVICE_HTTP: DWORD = 3;

/// A WinINet "connection"
/// 
/// Create it with `WinINet::connect`
pub struct WinINetConnection<'a>(&'a WinINet, HINTERNET);
impl Drop for WinINetConnection<'_> {
    fn drop(&mut self) {
        if self.1 == NULL {
            return;
        };
        unsafe {
            InternetCloseHandle(self.1);
        }
    }
}
pub const INTERNET_FLAG_RELOAD: DWORD = 0x80000000;
pub const INTERNET_FLAG_SECURE: DWORD = 0x00800000;
pub const INTERNET_FLAG_NO_CACHE_WRITE: DWORD = 0x04000000;
impl WinINetConnection<'_> {
    pub fn request(&self,
        method: &WinStr,
        path: &WinStr,
        tls: bool) -> Result<WinINetRequest, Error> {
        let mut wreq = WinINetRequest(self, NULL, None, ReqState::Send);
        let mut Flags = INTERNET_FLAG_RELOAD | INTERNET_FLAG_NO_CACHE_WRITE;
        if tls {
            Flags |= INTERNET_FLAG_SECURE;
        }
        let req = unsafe { HttpOpenRequest(self.1,
            from_win_str(method),
            from_win_str(path),
            NULL.cast(),                // Use default HTTP/1.1 as the version
            NULL.cast(),               // Do not provide any referrer
            NULL.cast(),           // Do not provide Accept types
            Flags,
            &wreq as *const WinINetRequest as DWORD_PTR)};
        if req == NULL {
            return Err(Error::last_os_error());
        }
        println!("WinINetRequest {:?} {:?}",&wreq as *const WinINetRequest, req);
        wreq.1 = req;//RequestHandle
        Ok(wreq)
    }
}

enum ReqState {
    Send,
    Upload,
    End,
    Download
}

/// A WinINet "request"
/// 
/// Create it with `WinINetConnection::request`
pub struct WinINetRequest<'a>(&'a WinINetConnection<'a>, HINTERNET, Option<Waker>, ReqState);
impl Drop for WinINetRequest<'_> {
    fn drop(&mut self) {
        if self.1 == NULL {
            return;
        };
        unsafe {
            InternetSetStatusCallback(self.1, None);
            InternetCloseHandle(self.1);
        }
    }
}
pub const ERROR_IO_PENDING: i32 = 997;
impl WinINetRequest<'_> {
    pub fn open(&self) -> Result<(), Error> {
        /*
        HttpSendRequest(ReqContext->RequestHandle,
                              NULL,                   // do not provide additional Headers
                              0,                      // dwHeadersLength
                              NULL,                   // Do not send any data
                              0);                     // dwOptionalLength
        */

        let mut lp_buffers_in = INTERNET_BUFFERS::new();

        if FALSE
            == unsafe {
                HttpSendRequestEx(
                    self.1,
                    &mut lp_buffers_in as *mut INTERNET_BUFFERS,
                    NULL as *mut INTERNET_BUFFERS,
                    0,//reserved
                    1,//ignored?
                )
            }
        {
            return Err(Error::last_os_error());
            //e.raw_os_error()==Some(ERROR_IO_PENDING)
        }
        println!("1");
        Ok(())
    }
    pub fn send(&self) -> Result<(), Error> {
        /*
        loop:
        Success = InternetWriteFile(self.1,);
        */
        if FALSE == unsafe { HttpEndRequest(self.1, NULL as *mut INTERNET_BUFFERS, 0, 0) } {
            return Err(Error::last_os_error());
        }
        println!("2");
        Ok(())
    }
    pub fn recv(&self) -> Result<(), Error> {
        //all sent, now recv
        //loop:
        let mut recv_body = vec![0u8; 9096];
        let mut read = 0u32;
        if FALSE == unsafe { InternetReadFile(self.1, recv_body.as_mut_ptr() as LPVOID, 9096, &mut read) } {
            return Err(Error::last_os_error());
        }
        println!("3");
        let data = String::from_utf8_lossy(&recv_body[..read as usize]);
        println!("{}", data);
        Ok(())
    }
}

use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::future::Future;
use std::marker::Unpin;

impl Future for WinINetRequest<'_> {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Self::Output>
    {

        let ret = match self.3 {
            ReqState::Send => {
                let mut lp_buffers_in = INTERNET_BUFFERS::new();
                unsafe {
                    HttpSendRequestEx(
                        self.1,
                        &mut lp_buffers_in as *mut INTERNET_BUFFERS,
                        NULL as *mut INTERNET_BUFFERS,
                        0,//reserved
                        1,//ignored?
                    )
                }
            },
            ReqState::Upload => todo!(),//AsyncWrite?
            ReqState::End => unsafe { HttpEndRequest(self.1, NULL as *mut INTERNET_BUFFERS, 0, 0) },
            ReqState::Download => todo!(),//AsyncRead?
        };

        if FALSE == ret {
            let e = Error::last_os_error();
            if e.raw_os_error() == Some(ERROR_IO_PENDING) {
                self.2 = Some(cx.waker().clone());
                return Poll::Pending;
            }
            return Poll::Ready(Err(e));
        }
        Poll::Ready(Ok(()))
    }
}
//open -> req
//req.await -> upstream
//upstream.write.await
//upstream.flush?.await -> download
//download.read.await
