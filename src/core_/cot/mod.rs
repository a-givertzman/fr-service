//!
//! #### Couse of transmission.
//! Bitmask (Enum) being a part of the [Point](../point/index.html).  
//! Contains information about transmission cause and direction.  
//! Basic values at the moment (can be extended):
//! ```
//! Inf      = 0b_0000_0010; // 2   (0x2);
//! Act      = 0b_0000_0100; // 4   (0x4);
//! ActCon   = 0b_0000_1000; // 8   (0x8);
//! ActErr   = 0b_0001_0000; // 16  (0x10);
//! Req      = 0b_0010_0000; // 32  (0x20);
//! ReqCon   = 0b_0100_0000; // 64  (0x40);
//! ReqErr   = 0b_1000_0000; // 128 (0x80);
//! Read     = 0b_1101_1010; // 218 (0xDA)
//! Write    = 0b_0010_0100; // 36  (0x24)
//! All      = 0b_1111_1111; // 255 (0xFF)
//!
pub mod cot;