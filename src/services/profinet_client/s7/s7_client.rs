#![allow(non_snake_case)]

use log::{debug, error, info, trace, warn, LevelFilter};
use snap7_sys::S7Object;
use std::ffi::CString;
use std::ffi::{c_void, c_int};
// use std::os::raw::{c_int, c_void};
use std::time::Duration;

use super::s7_error::S7Error;
use super::s7_lib::S7LIB;


#[derive(Debug)]
pub struct S7Client {
    pub id: String,
    ip: CString,
    handle: S7Object,
    req_len: usize,
    neg_len: usize,
    pub isConnected: bool,
    // reconnectDelay: Duration,
}
impl S7Client {
    pub fn new(parent: impl Into<String>, ip: String) -> Self {
        Self {
            id: format!("{}/S7Client({})", parent.into(), ip),
            ip: CString::new(ip).unwrap(),
            handle: unsafe { S7LIB.Cli_Create() },
            req_len: 0,
            neg_len: 0,
            isConnected: false,
            // reconnectDelay: match reconnectDelay {
            //     Some(delay) => delay,
            //     None => Duration::from_secs(3),
            // },
        }
    }
    pub fn connect(&mut self) -> Result<(), S7Error> {
        let mut req: c_int = 0;
        let mut neg: c_int = 0;
        let mut errCode = 0;
        unsafe {
            // #[warn(temporary_cstring_as_ptr)]
            errCode = S7LIB.Cli_ConnectTo(self.handle, self.ip.as_ptr(), 0, 1);
            S7LIB.Cli_GetPduLength(self.handle, &mut req, &mut neg);
            self.req_len = req as usize;
            self.neg_len = neg as usize;
        }
        if errCode == 0 {
            self.isConnected = true;
            debug!("{}.connect | successfully connected", self.id);
            Ok(())
        } else {
            self.isConnected = false;
            let err = S7Error::from(errCode);
            if log::max_level() == LevelFilter::Trace {
                warn!("{}.connect | connection error: {:?}", self.id, err);
            }
            Err(err)
            // thread::sleep(self.reconnectDelay);
        }
        // while !self.isConnected {
        // }
    }
    pub fn read(&self, dbNum: u32, start: u32, size: u32) -> Result<Vec<u8>, String> {
        let mut buf = Vec::<u8>::new();
        buf.resize(size as usize, 0);
        let res;
        unsafe {
            res = S7LIB.Cli_DBRead(
                self.handle,
                dbNum as c_int,
                start as c_int,
                size as c_int,
                buf.as_mut_ptr() as *mut c_void,
            ) as i32;
        }
        if res == 0 {
            Ok(buf)
        } else {
            Err(String::from(S7Error::text(res)))
        }
    }
    pub fn close(&mut self) {
        unsafe {
            S7LIB.Cli_Disconnect(self.handle);
        }
    }
}
impl Drop for S7Client {
    fn drop(&mut self) {
        self.close();
        unsafe {
            S7LIB.Cli_Destroy(&mut self.handle);
        }
    }
}
