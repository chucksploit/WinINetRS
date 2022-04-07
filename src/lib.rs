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

pub type HINTERNET = LPVOID;
pub type LPHINTERNET = *mut HINTERNET;

#[allow(non_camel_case_types)]
#[cfg(feature = "unicode")]
mod unicode;
#[cfg(feature = "unicode")]
use unicode::{
    FromWinStr, FromWinStrOrNull, HttpEndRequestW as HttpEndRequest,
    HttpSendRequestExW as HttpSendRequestEx, InternetConnectW as InternetConnect,
    InternetOpenW as InternetOpen, InternetSetStatusCallbackW as InternetSetStatusCallback,
    HttpOpenRequestW as HttpOpenRequest, INTERNET_BUFFERSW as INTERNET_BUFFERS
};
#[cfg(feature = "unicode")]
pub use unicode::WinStr;

#[cfg(not(feature = "unicode"))]
mod ansi;
#[cfg(not(feature = "unicode"))]
use ansi::{
    FromWinStr, FromWinStrOrNull, HttpEndRequestA as HttpEndRequest,
    HttpSendRequestExA as HttpSendRequestEx, InternetConnectA as InternetConnect,
    InternetOpenA as InternetOpen, InternetSetStatusCallbackA as InternetSetStatusCallback,
    HttpOpenRequestA as HttpOpenRequest, INTERNET_BUFFERSA as INTERNET_BUFFERS
};
#[cfg(not(feature = "unicode"))]
pub use ansi::WinStr;

#[repr(u32)] //DWORD
pub enum InternetStatus {
    RESOLVING_NAME = 10,
    NAME_RESOLVED = 11,
    CONNECTING_TO_SERVER = 20,
    CONNECTED_TO_SERVER = 21,
    SENDING_REQUEST = 30,
    REQUEST_SENT = 31,
    RECEIVING_RESPONSE = 40,
    RESPONSE_RECEIVED = 41,
    CTL_RESPONSE_RECEIVED = 42,
    PREFETCH = 43,
    CLOSING_CONNECTION = 50,
    CONNECTION_CLOSED = 51,
    HANDLE_CREATED = 60,
    HANDLE_CLOSING = 70,
    DETECTING_PROXY = 80,
    REQUEST_COMPLETE = 100,
    REDIRECT = 110,
    INTERMEDIATE_RESPONSE = 120,
    USER_INPUT_REQUIRED = 140,
    STATE_CHANGE = 200,
    COOKIE_SENT = 320,
    COOKIE_RECEIVED = 321,
    PRIVACY_IMPACTED = 324,
    P3P_HEADER = 325,
    P3P_POLICYREF = 326,
    COOKIE_HISTORY = 327,
}

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

#[link(name = ":libwininet.a")]
extern "system" {
    pub fn InternetQueryDataAvailable(
        hFile: HINTERNET,
        lpdwNumberOfBytesAvailable: LPDWORD,
        dwFlags: DWORD,
        dwContext: DWORD_PTR,
    ) -> BOOL;
    pub fn InternetReadFile(
        hFile: HINTERNET,
        lpBuffer: LPVOID,
        dwNumberOfBytesToRead: DWORD,
        lpdwNumberOfBytesRead: LPDWORD,
    ) -> BOOL;
    pub fn InternetSetFilePointer(
        hFile: HINTERNET,
        lDistanceToMove: LONG,
        lpDistanceToMoveHigh: PLONG,
        dwMoveMethod: DWORD,
        dwContext: DWORD_PTR,
    ) -> DWORD;
    pub fn InternetWriteFile(
        hFile: HINTERNET,
        lpBuffer: LPCVOID,
        dwNumberOfBytesToWrite: DWORD,
        lpdwNumberOfBytesWritten: LPDWORD,
    ) -> BOOL;

    pub fn InternetCloseHandle(hInternet: HINTERNET) -> BOOL;
}

#[repr(u32)] //DWORD
pub enum InternetOpenType {
    ///Retrieves the proxy or direct configuration from the registry.
    PRECONFIG = 0,
    ///Resolves all host names locally.
    DIRECT = 1,
    ///Use Proxy
    PROXY = 3,
    ///PRECONFIG without INS
    PRECONFIG_WITH_NO_AUTOPROXY = 4,
}
const INTERNET_FLAG_ASYNC: DWORD = 0x10000000;
const INTERNET_INVALID_STATUS_CALLBACK: isize = -1;
const NULL: *mut c_void = 0 as *mut c_void;
use std::io::Error;

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
                FromWinStr(agent),
                open_type as DWORD,
                FromWinStrOrNull(proxy),
                FromWinStrOrNull(direct_hosts),
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
                FromWinStr(host),
                port,
                FromWinStrOrNull(user),
                FromWinStrOrNull(password),
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
        let mut dwFlags = INTERNET_FLAG_RELOAD | INTERNET_FLAG_NO_CACHE_WRITE;
        if tls {
            dwFlags |= INTERNET_FLAG_SECURE;
        }
        let req = unsafe { HttpOpenRequest(self.1,
            FromWinStr(method),
            FromWinStr(path),
            NULL.cast(),                // Use default HTTP/1.1 as the version
            NULL.cast(),               // Do not provide any referrer
            NULL.cast(),           // Do not provide Accept types
            dwFlags,
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

        let mut lpBuffersIn = INTERNET_BUFFERS::new();

        if FALSE
            == unsafe {
                HttpSendRequestEx(
                    self.1,
                    &mut lpBuffersIn as *mut INTERNET_BUFFERS,
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
                let mut lpBuffersIn = INTERNET_BUFFERS::new();
                unsafe {
                    HttpSendRequestEx(
                        self.1,
                        &mut lpBuffersIn as *mut INTERNET_BUFFERS,
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
