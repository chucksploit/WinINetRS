/// https://docs.microsoft.com/en-us/windows/win32/wininet/asynchronous-example-application


#[cfg(not(feature = "unicode"))]
use std::ffi::{OsStr, CString};

#[cfg(feature = "unicode")]
use widestring::U16CString;

use win_inet::{InternetOpenType, WinINet, WinINetRequest, InternetStatus, INTERNET_ASYNC_RESULT};

fn main() {
    if let Err(e) = work() {
        eprintln!("{}", e);
    }
}
use std::sync::Barrier;
static mut barrier: Option<Barrier> = None;

fn work() -> std::io::Result<()> {
  unsafe {
    barrier = Some(Barrier::new(2));
  }

    #[cfg(feature = "unicode")]
    let (agent, host, method, path) = (
      U16CString::from_str("Hello, world!").unwrap(),
      U16CString::from_str("drak.li").unwrap(),
      U16CString::from_str("GET").unwrap(),
      U16CString::from_str("/").unwrap()
    );
    #[cfg(not(feature = "unicode"))]
    let (agent, host, method, path) = (
      CString::new("Hello, world!").unwrap(),
      CString::new("drak.li").unwrap(),
      CString::new("GET").unwrap(),
      CString::new("/").unwrap()
    );

    let mut agent = WinINet::new(
      &agent,
      InternetOpenType::Preconfig,
      None,
      None,
    )?;
    agent.set_callback(Some(callback));
    let c = agent.connect(&host, 80, None, None)?;
    let r = c.request(&method, &path, false)?;
    
    println!("open...");
    if let Err(e) = r.open() {
      if e.raw_os_error() != Some(win_inet::ERROR_IO_PENDING) {
        return Err(e);
      }
      println!("wait");
      unsafe {barrier.as_ref()}.unwrap().wait();
    }

    println!("send...");
    unsafe {
      barrier = Some(Barrier::new(2));
    }
    if let Err(e) = r.send() {
      if e.raw_os_error() != Some(win_inet::ERROR_IO_PENDING) {
        return Err(e);
      }
      println!("wait2");
      unsafe {barrier.as_ref()}.unwrap().wait();
    }
    println!("recv...");
    unsafe {
      barrier = Some(Barrier::new(2));
    }
    if let Err(e) = r.recv() {
      if e.raw_os_error() != Some(win_inet::ERROR_IO_PENDING) {
        return Err(e);
      }
      println!("wait3");
      unsafe {barrier.as_ref()}.unwrap().wait();

    }
    Ok(())
}
unsafe extern "system" fn callback(
    handle: win_inet::HINTERNET,
    context: usize,
    status: u32,
    info: win_inet::LPVOID,
    info_len: u32,
) {
    if status == InternetStatus::RequestComplete as u32 {
      let ares = &*(info as *mut INTERNET_ASYNC_RESULT);
      println!("{} Request complete {} {:?}", context, ares.dwError, handle);
      if let Some(w) = barrier.as_ref() {
        w.wait();
      }
      //let r = &*(context as *const WinINetRequest);
      return;
    }
    if status == InternetStatus::HandleCreated as u32 {
      let ares = &*(info as *mut INTERNET_ASYNC_RESULT);
      println!("{} Handle created {}", context, ares.dwError);
      return;
    }
    println!(
        "{} {} {:?} {} {:?}",
        context, status, info, info_len, handle
    );
}