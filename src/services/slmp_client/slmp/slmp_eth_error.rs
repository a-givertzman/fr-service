/// SLMP documented Ethernet module error codes
/// - Please refer to code of the function ErrorText() for the explanation
/// - source: design/protocol-slmp/jy997d56001k.pdf
#[derive(Debug)]
#[repr(i64)]
pub enum SlmpEthError {
    DeviceNotFound                              = 0xC035,
    CommunicationDataCodeSetAsciiError          = 0xC050,
    MaximumNumberOfBitDevicesExceeds            = 0xC051,
    MaximumNumberOfWordDevicesExceeds           = 0xC052,
    MaximumNumberOfBitRandomDevicesExceeds      = 0xC053,
    MaximumNumberOfWordRandomDevicesExceeds     = 0xC054,
    MaximumAddressExceeds                       = 0xC056,
    SlmpRequestDataLengthError                  = 0xC057,
    RequestAsciiToBinaryConversionLengthError   = 0xC058,
    CommandError                                = 0xC059,
    ReadWriteDeviceError                        = 0xC05B,
    WordDeviceRequestContentsError              = 0xC05C,
    RequestCannotBeExecutedForCpuModule         = 0xC05F,
    BitDeviceRequestContentsError               = 0xC060,
    RequestDataLengthError                      = 0xC061,
    CommunicationDataCodeSetBinary              = 0xC06F,
    BlocksNumberExceeds                         = 0xC0D8,
    Inner(String),
}
//
// 
impl SlmpEthError {
    pub fn text(code: i32) -> String {
        let as_str = &format!("{}", code);
        let err = match code {
            0xC035 => "The existence of the external device could not be confirmed within the response monitoring timer value.",
            0xC050 => "When the communication data code is set to “ASCII”, ASCII code data which cannot be converted to binary is received.",
            0xC051 => "Maximum number of bit devices for which data can be read/written all at once is outside the allowable range.",
            0xC052 => "Maximum number of word devices for which data can be read/written all at once is outside the allowable range.",
            0xC053 => "Maximum number of bit devices for which data can be random read/written all at once is outside the allowable range.",
            0xC054 => "Maximum number of word devices for which data can be random read/written all at once is outside the allowable range.",
            0xC056 => "Read or write request exceeds maximum address.",
            0xC057 => "The request data length in the SLMP message does not match the number of data in the character section (part of the test).",
            0xC058 => "Request data length after ASCII-to-binary conversion does not match the number of data in the character section (part of text).",
            0xC059 => "- Error in command or subcommand specification.\n- There is a command or subcommand that cannot be used by the CPU module.",
            0xC05B => "CPU module cannot read or write from/to specified device.",
            0xC05C => "Error in request contents. (Error related to device specification, such as reading or writing by bit unit for word device, etc.)",
            0xC05E => "The communication time between the Ethernet module and PLC CPU exceeds the Ethernet monitor timer setting.",
            0xC05F => "There is a request that cannot be executed for the target CPU module.",
            0xC060 => "Error in request contents. (Error in specification of data for bit device, etc.)",
            0xC061 => "Request data length does not match the number of data in the character section (part of text).",
            0xC06F => "When the communication data code is set to “Binary”, a request message of ASCII is received. (Error history of this error code is registered but no error response is sent.)",
            0xC0D8 => "The number of specified blocks exceeds the range.",                    
            _ => as_str,
        };
        err.to_owned()
    }    
}
//
// 
impl From<i32> for SlmpEthError {
    fn from(value: i32) -> Self {
        match value {
            0xC035 => Self::DeviceNotFound,
            0xC050 => Self::CommunicationDataCodeSetAsciiError,
            0xC051 => Self::MaximumNumberOfBitDevicesExceeds,
            0xC052 => Self::MaximumNumberOfWordDevicesExceeds,
            0xC053 => Self::MaximumNumberOfBitRandomDevicesExceeds,
            0xC054 => Self::MaximumNumberOfWordRandomDevicesExceeds,
            0xC056 => Self::MaximumAddressExceeds,
            0xC057 => Self::SlmpRequestDataLengthError,
            0xC058 => Self::RequestAsciiToBinaryConversionLengthError,
            0xC059 => Self::CommandError,
            0xC05B => Self::ReadWriteDeviceError,
            0xC05C => Self::WordDeviceRequestContentsError, 
            0xC05F => Self::RequestCannotBeExecutedForCpuModule,
            0xC060 => Self::BitDeviceRequestContentsError,
            0xC061 => Self::RequestDataLengthError,
            0xC06F => Self::CommunicationDataCodeSetBinary,
            0xC0D8 => Self::BlocksNumberExceeds,
            _ => {
                Self::Inner(format!("{} ({})", Self::text(value), value))
            }
        }
    }
}