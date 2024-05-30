#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::services::slmp_client::{self, slmp::{self, c_slmp_client::{self, CSlmpInfo, FrameType, ProcessorNumber, SLMP_MakePacketStream, SlmpCommand, SlmpSubCommand, TimerValue}, slmp_device_code::SlmpDeviceCode}};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing such functionality / behavior
    #[test]
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let deviceCode = SlmpDeviceCode::D;
        let offset: usize = 0;
        let wordCount: u16 = 0;
        let usSerialNumber = 0;
        let usNetNumber = 0;
        let usNodeNumber = 0xFF;
        let usDataLength = 0;
        let usEndCode = 0x0000;
        let slmpHeadDevice = &offset.to_le_bytes()[..3];
        let slmpWordCount = wordCount.to_le_bytes();
        let pucData = [slmpHeadDevice, &[deviceCode as u8], &slmpWordCount].concat();
        let slmp_info = CSlmpInfo::new(
            FrameType::BinReqSt,
            usSerialNumber,
            usNetNumber,
            usNodeNumber,
            ProcessorNumber::CpuDefault,
            usDataLength,
            TimerValue::WaitForever,
            SlmpCommand::DeviceRead,
            SlmpSubCommand::SubWord0,
            usEndCode,
            &pucData,
        );
        let slmp_make_packet_stream = unsafe { SLMP_MakePacketStream(
            FrameType::BinReqSt as u64, 
            &slmp_info, 
            pucData.as_ptr(),
        ) };
        println!("slmp_make_packet_stream: {}", slmp_make_packet_stream);
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}
