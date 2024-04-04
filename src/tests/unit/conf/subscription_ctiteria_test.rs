#[cfg(test)]

mod subscription_criteria {
    use concat_string::concat_string;
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{core_::cot::cot::Cot, services::multi_queue::subscription_criteria::SubscriptionCriteria};
    ///
    /// 
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing SubscriptionCriteria::destination() functionality / behavior
    #[test]
    fn destination() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new("template_test", Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.MainMode"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.MainMode"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.ActiveWinch"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.ActiveWinch"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.Winch1Mode"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.Winch1Mode"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.WaveHeightLevel"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.WaveHeightLevel"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.ConstantTensionLevel"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.ConstantTensionLevel"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.SetRelativeDepth"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.SetRelativeDepth"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.ResetRelativeDepth"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.ResetRelativeDepth"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.RotateLeft"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.RotateLeft"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.RotateRight"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.RotateRight"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.OutreachFwd"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.OutreachFwd"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.OutreachRev"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.OutreachRev"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.JibUp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.JibUp"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.JibDown"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.JibDown"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.BoomUp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.BoomUp"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.BoomDown"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.BoomDown"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.WinchUp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.WinchUp"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.WinchDown"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.WinchDown"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.MainMode"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.MainMode"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.ActiveWinch"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.ActiveWinch"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Winch1Mode"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Winch1Mode"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.WaveHeightLevel"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.WaveHeightLevel"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.CraneOffshore"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.CraneOffshore"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.ParkingModeActive"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.ParkingModeActive"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.MOPS"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.MOPS"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.AOPS"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.AOPS"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.AHC"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.AHC"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.WinchBrake"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.WinchBrake"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.SWLProtection"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.SWLProtection"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Reserve1"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Reserve1"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Reserve2"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Reserve2"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Reserve3"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Reserve3"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Level"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Level"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Active"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Active"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Reserve1"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Reserve1"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Reserve2"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Reserve2"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Reserve3"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Reserve3"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Hook.X"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Hook.X"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Hook.Y"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Hook.Y"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Hook.Speed"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Hook.Speed"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Radius"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Radius"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Depth"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Depth"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.DeckDepth"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.DeckDepth"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Wind"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Wind"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Pitch"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Pitch"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Roll"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Roll"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Slewing"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Slewing"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.BoomAngle"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.BoomAngle"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.JibAngle"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.JibAngle"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Reserve1"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Reserve1"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Reserve2"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Reserve2"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Reserve3"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Reserve3"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch1.SWL0"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch1.SWL0"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch1.SWL"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch1.SWL"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch1.Load"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch1.Load"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch2.SWL0"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch2.SWL0"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch2.SWL"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch2.SWL"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch2.Load"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch2.Load"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch3.SWL0"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch3.SWL0"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch3.SWL"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch3.SWL"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch3.Load"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch3.Load"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.Pump1.State"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.Pump1.State"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.Pump2.State"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.Pump2.State"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.EmergencyHPU.State"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.EmergencyHPU.State"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.OilTemp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.OilTemp"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.ValveEV1"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.ValveEV1"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.ValveEV2"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.ValveEV2"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.LimitSwitchDown"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.LimitSwitchDown"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor1Active"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor1Active"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor2Active"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor2Active"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR1"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR1"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR2"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR2"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.LVDT1"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.LVDT1"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.LVDT2"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.LVDT2"           ) 
        ];
        for (cot, name) in test_data {
            let criteria = SubscriptionCriteria::new(name, cot);
            let result = criteria.destination();
            let target = match cot {
                Cot::All => name.to_owned(),
                _        => concat_string!(cot.as_str(), ":", name),
            };
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing SubscriptionCriteria::destination() functionality / behavior
    #[test]
    #[ignore = "Performance test"]
    fn performance() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new("template_test", Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.MainMode"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.MainMode"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.ActiveWinch"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.ActiveWinch"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.Winch1Mode"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.Winch1Mode"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.WaveHeightLevel"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.WaveHeightLevel"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.ConstantTensionLevel"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.ConstantTensionLevel"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.SetRelativeDepth"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.SetRelativeDepth"),
            (Cot::ReqCon, "/App/ied12/db902_panel_controls/Settings.CraneMode.ResetRelativeDepth"),
            (Cot::ReqErr, "/App/ied12/db902_panel_controls/Settings.CraneMode.ResetRelativeDepth"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.RotateLeft"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.RotateLeft"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.RotateRight"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.RotateRight"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.OutreachFwd"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.OutreachFwd"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.OutreachRev"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.OutreachRev"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.JibUp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.JibUp"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.JibDown"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.JibDown"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.BoomUp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.BoomUp"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.BoomDown"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.BoomDown"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.WinchUp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.WinchUp"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMovement.WinchDown"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMovement.WinchDown"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.MainMode"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.MainMode"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.ActiveWinch"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.ActiveWinch"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Winch1Mode"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Winch1Mode"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.WaveHeightLevel"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.WaveHeightLevel"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.CraneOffshore"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.CraneOffshore"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.ParkingModeActive"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.ParkingModeActive"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.MOPS"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.MOPS"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.AOPS"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.AOPS"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.AHC"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.AHC"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.WinchBrake"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.WinchBrake"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.SWLProtection"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.SWLProtection"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Reserve1"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Reserve1"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Reserve2"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Reserve2"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/CraneMode.Reserve3"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/CraneMode.Reserve3"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Level"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Level"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Active"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Active"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Reserve1"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Reserve1"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Reserve2"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Reserve2"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/ConstantTension.Reserve3"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/ConstantTension.Reserve3"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Hook.X"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Hook.X"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Hook.Y"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Hook.Y"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Hook.Speed"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Hook.Speed"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Radius"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Radius"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Depth"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Depth"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.DeckDepth"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.DeckDepth"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Wind"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Wind"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Pitch"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Pitch"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Roll"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Roll"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Slewing"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Slewing"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.BoomAngle"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.BoomAngle"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.JibAngle"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.JibAngle"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Reserve1"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Reserve1"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Reserve2"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Reserve2"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Crane.Reserve3"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Crane.Reserve3"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch1.SWL0"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch1.SWL0"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch1.SWL"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch1.SWL"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch1.Load"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch1.Load"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch2.SWL0"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch2.SWL0"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch2.SWL"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch2.SWL"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch2.Load"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch2.Load"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch3.SWL0"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch3.SWL0"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch3.SWL"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch3.SWL"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/Winch3.Load"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/Winch3.Load"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.Pump1.State"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.Pump1.State"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.Pump2.State"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.Pump2.State"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.EmergencyHPU.State"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.EmergencyHPU.State"),
            (Cot::ReqCon, "/App/ied14/db906_visual_data/HPU.OilTemp"),
            (Cot::ReqErr, "/App/ied14/db906_visual_data/HPU.OilTemp"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.ValveEV1"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.ValveEV1"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.ValveEV2"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.ValveEV2"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.LimitSwitchDown"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.LimitSwitchDown"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor1Active"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor1Active"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor2Active"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.Hydromotor2Active"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR1"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR1"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR2"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.EncoderBR2"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.LVDT1"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.LVDT1"),
            (Cot::ReqCon, "/App/ied13/db905_visual_data_fast/Winch.LVDT2"),
            (Cot::ReqErr, "/App/ied13/db905_visual_data_fast/Winch.LVDT2"           ) 
        ];
        let iterations = 100_000;
        build_criteria(iterations, &test_data);
        build_concat_string(iterations, &test_data);
        criteria_dest(iterations, &test_data);
        concat_string_dest(iterations, &test_data);
        test_duration.exit();
    }
    fn build_criteria(iterations: usize, test_data: &[(Cot, &str)]) {
        let time = Instant::now();
        for _ in 0..iterations {
            let mut results = vec![];
            for (cot, name) in test_data {
                let criteria = SubscriptionCriteria::new(*name, *cot).destination();
                results.push(criteria);
                // let result = criteria.destination();
                // let target = match cot {
                //     Cot::All => name.to_owned(),
                //     _        => concat_string!(cot.as_str(), ":", name),
                // };
                // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            }
        }
        println!("Build criteria       elapsed: {:?}", time.elapsed());
    }
    fn build_concat_string(iterations: usize, test_data: &[(Cot, &str)]) {
        let time = Instant::now();
        for _ in 0..iterations {
            let mut results = vec![];
            for (cot, name) in test_data {
                // let criteria = SubscriptionCriteria::new(name, cot);
                // let result = criteria.destination();
                let target = match cot {
                    Cot::All => name.to_string(),
                    _        => concat_string!(cot.as_str(), ":", *name),
                };
                results.push(target);
                // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            }
        }
        println!("Build concat_string  elapsed: {:?}", time.elapsed());
    }
    fn criteria_dest(iterations: usize, test_data: &[(Cot, &str)]) {
        let time = Instant::now();
        let (cot, name) = test_data[0];
        let criteria = SubscriptionCriteria::new(name, cot);
        let mut results = vec![];
        for _ in 0..iterations {
            let dest = criteria.destination();
            results.push(dest);
        }
        println!("Criteria      dest       elapsed: {:?}", time.elapsed());
    }
    fn concat_string_dest(iterations: usize, test_data: &[(Cot, &str)]) {
        let time = Instant::now();
        let (cot, name) = test_data[0];
        let mut results = vec![];
        for _ in 0..iterations {
            let dest = match cot {
                Cot::All => name.to_string(),
                _        => concat_string!(cot, ":", name),
            };
            results.push(dest);
        }
        println!("concat_string dest       elapsed: {:?}", time.elapsed());
    }
}
