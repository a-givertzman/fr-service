#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use log::{error, info};
use once_cell::sync::Lazy;
use snap7_sys::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::time::Duration;

static S7LIB: Lazy<LibSnap7> = Lazy::new(|| {
    println!("initializing LibSnap7 lib...");
    unsafe { LibSnap7::new("/usr/lib/libsnap7.so") }.unwrap()
});


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
    pub fn new(parent: impl Into<String>, ip: String, reconnectDelay: Option<Duration>) -> Self {
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
            info!("{}.connect | successfully connected", self.id);
            Ok(())
        } else if errCode > 0 {
            self.isConnected = false;
            let err = S7Error::from(errCode as u32);
            error!("{}.connect | connection error: {:?}", self.id, err);
            Err(err)
            // thread::sleep(self.reconnectDelay);
        } else {
            let err = S7Error::Unknown;
            error!("{}.connect | connection error: {:?} ({}) - unknown error code", self.id, err, errCode);
            Err(err)
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
            Err(String::from(error_text(res)))
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
pub fn error_text(code: i32) -> String {
    let mut err = Vec::<u8>::new();
    err.resize(1024, 0);
    unsafe {
        S7LIB.Cli_ErrorText(
            code as c_int,
            err.as_mut_ptr() as *mut c_char,
            err.len() as c_int,
        );
    }
    if let Some(i) = err.iter().position(|&r| r == 0) {
        err.truncate(i);
    }
    let err = unsafe { std::str::from_utf8_unchecked(&err) };
    err.to_owned()
}


/// Snap7 documented Error codes
/// - Please refer to code of the function ErrorText() for the explanation
/// - source: https://snap7.sourceforge.net/sharp7.html
#[derive(Debug)]
pub enum S7Error {
    TCPSocketCreation         = 0x00000001,
    TCPConnectionTimeout      = 0x00000002,
    TCPConnectionFailed       = 0x00000003,
    TCPReceiveTimeout         = 0x00000004,
    TCPDataReceive            = 0x00000005,
    TCPSendTimeout            = 0x00000006,
    TCPDataSend               = 0x00000007,
    TCPConnectionReset        = 0x00000008,
    TCPNotConnected           = 0x00000009,
    TCPUnreachableHost        = 0x00002751,
    IsoConnect                = 0x00010000,
    IsoInvalidPDU             = 0x00030000,
    IsoInvalidDataSize        = 0x00040000,
    CliNegotiatingPDU         = 0x00100000,
    CliInvalidParams          = 0x00200000,
    CliJobPending             = 0x00300000,
    CliTooManyItems           = 0x00400000,
    CliInvalidWordLen         = 0x00500000,
    CliPartialDataWritten     = 0x00600000,
    CliSizeOverPDU            = 0x00700000,
    CliInvalidPlcAnswer       = 0x00800000,
    CliAddressOutOfRange      = 0x00900000,
    CliInvalidTransportSize   = 0x00A00000,
    CliWriteDataSizeMismatch  = 0x00B00000,
    CliItemNotAvailable       = 0x00C00000,
    CliInvalidValue           = 0x00D00000,
    CliCannotStartPLC         = 0x00E00000,
    CliAlreadyRun             = 0x00F00000,
    CliCannotStopPLC          = 0x01000000,
    CliCannotCopyRamToRom     = 0x01100000,
    CliCannotCompress         = 0x01200000,
    CliAlreadyStop            = 0x01300000,
    CliFunNotAvailable        = 0x01400000,
    CliUploadSequenceFailed   = 0x01500000,
    CliInvalidDataSizeRecvd   = 0x01600000,
    CliInvalidBlockType       = 0x01700000,
    CliInvalidBlockNumber     = 0x01800000,
    CliInvalidBlockSize       = 0x01900000,
    CliNeedPassword           = 0x01D00000,
    CliInvalidPassword        = 0x01E00000,
    CliNoPasswordToSetOrClear = 0x01F00000,
    CliJobTimeout             = 0x02000000,
    CliPartialDataRead        = 0x02100000,
    CliBufferTooSmall         = 0x02200000,
    CliFunctionRefused        = 0x02300000,
    CliDestroying             = 0x02400000,
    CliInvalidParamNumber     = 0x02500000,
    CliCannotChangeParam      = 0x02600000,
    CliFunctionNotImplemented = 0x02700000,
    Unknown = 0x99900000,
}
impl From<u32> for S7Error {
    fn from(value: u32) -> Self {
        match value {
            0x00000001 => Self::TCPSocketCreation,
            0x00000002 => Self::TCPConnectionTimeout,
            0x00000003 => Self::TCPConnectionFailed,
            0x00000004 => Self::TCPReceiveTimeout,
            0x00000005 => Self::TCPDataReceive,
            0x00000006 => Self::TCPSendTimeout,
            0x00000007 => Self::TCPDataSend,
            0x00000008 => Self::TCPConnectionReset,
            0x00000009 => Self::TCPNotConnected,
            0x00002751 => Self::TCPUnreachableHost,
            0x00010000 => Self::IsoConnect,
            0x00030000 => Self::IsoInvalidPDU,
            0x00040000 => Self::IsoInvalidDataSize,
            0x00100000 => Self::CliNegotiatingPDU,
            0x00200000 => Self::CliInvalidParams,
            0x00300000 => Self::CliJobPending,
            0x00400000 => Self::CliTooManyItems,
            0x00500000 => Self::CliInvalidWordLen,
            0x00600000 => Self::CliPartialDataWritten,
            0x00700000 => Self::CliSizeOverPDU,
            0x00800000 => Self::CliInvalidPlcAnswer,
            0x00900000 => Self::CliAddressOutOfRange,
            0x00A00000 => Self::CliInvalidTransportSize,
            0x00B00000 => Self::CliWriteDataSizeMismatch,
            0x00C00000 => Self::CliItemNotAvailable,
            0x00D00000 => Self::CliInvalidValue,
            0x00E00000 => Self::CliCannotStartPLC,
            0x00F00000 => Self::CliAlreadyRun,
            0x01000000 => Self::CliCannotStopPLC,
            0x01100000 => Self::CliCannotCopyRamToRom,
            0x01200000 => Self::CliCannotCompress,
            0x01300000 => Self::CliAlreadyStop,
            0x01400000 => Self::CliFunNotAvailable,
            0x01500000 => Self::CliUploadSequenceFailed,
            0x01600000 => Self::CliInvalidDataSizeRecvd,
            0x01700000 => Self::CliInvalidBlockType,
            0x01800000 => Self::CliInvalidBlockNumber,
            0x01900000 => Self::CliInvalidBlockSize,
            0x01D00000 => Self::CliNeedPassword,
            0x01E00000 => Self::CliInvalidPassword,
            0x01F00000 => Self::CliNoPasswordToSetOrClear,
            0x02000000 => Self::CliJobTimeout,
            0x02100000 => Self::CliPartialDataRead,
            0x02200000 => Self::CliBufferTooSmall,
            0x02300000 => Self::CliFunctionRefused,
            0x02400000 => Self::CliDestroying,
            0x02500000 => Self::CliInvalidParamNumber,
            0x02600000 => Self::CliCannotChangeParam,
            0x02700000 => Self::CliFunctionNotImplemented,
            _ => Self::Unknown,
        }
    }
}