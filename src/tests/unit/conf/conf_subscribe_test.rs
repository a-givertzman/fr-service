#[cfg(test)]

mod conf_subscribe {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::conf::{conf_subscribe::ConfSubscribe, conf_tree::ConfTree, point_config::point_config::PointConfig};
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
    /// 
    #[test]
    fn new() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let points = [
            r#"point Drive.Speed: 
                type: 'Real'
                address:
                    offset: 0"#,
            r#"point Drive.OutputVoltage: 
                type: 'Real'
                address:
                    offset: 4"#,
            r#"point Drive.DCVoltage: 
                type: 'Real'
                address:
                    offset: 8"#,
            r#"point Drive.Current: 
                type: 'Real'
                address:
                    offset: 12
                history: r"#,
            r#"point Drive.Torque: 
                type: 'Real'
                address:
                    offset: 16"#,
        ];
        let points = points.map(|conf| {
            let conf = serde_yaml::from_str(conf).unwrap();
            PointConfig::from_yaml(self_id, &conf)
        });
        let test_data = [
            r#"
                subscribe: MultiQueue
            "#,
            r#"
                subscribe:
                    MultiQueue_01: {}
                    MultiQueue_02: {}
            "#,
            r#"
                subscribe: 
                    MultiQueue:
                        Inf: []
            "#,
            r#"
                subscribe: 
                    MultiQueue:
                        {cot: Inf, history: r}: []
            "#,
            r#"
                subscribe: 
                    MultiQueue:
                        {cot: Inf, history: r}:
                            - /App/Service/Point.Name.01
                            - /App/Service/Point.Name.02
            "#,
        ];
        for conf in test_data {
            match serde_yaml::from_str(conf) {
                Ok(conf) => {
                    let conf: serde_yaml::Value = conf;
                    let (_key, conf) = conf.as_mapping().unwrap().into_iter().next().unwrap();
                    // let conf = ConfTree::new(key.as_str().unwrap(), conf.clone());
                    let conf_subscribe = ConfSubscribe::new(conf.clone(), points.to_vec());
                    println!("\nconf          : {:#?}", conf);
                    println!("conf_subscribe: {:#?}", conf_subscribe);
                },
                Err(err) => {
                    println!("Deserialize error: {:#?}", err);
                },
            };
        }
            
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    /// 
    // #[test]
    fn with() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let conf = [
            r#"
            subscribe: MultiQueue
#            service ProfinetClient Ied01:
#                cycle: 500 ms                         # operating cycle time of the module
            "#,
            r#"
            subscribe:
                MultiQueue_01: {}
                MultiQueue_02: {}
#            service ProfinetClient Ied01:
#                cycle: 500 ms                         # operating cycle time of the module
            "#,
            r#"
            subscribe: 
                MultiQueue:
                    Inf: []
            #service ProfinetClient Ied01:
            #    cycle: 500 ms                         # operating cycle time of the module
            "#,
            r#"
            subscribe: 
                MultiQueue:
                    {cot: Inf, history: r}: '*'
            #service ProfinetClient Ied01:
            #    cycle: 500 ms                         # operating cycle time of the module
            "#,
            r#"
            subscribe: 
                MultiQueue:
                    {cot: Inf, history: r}:
                        - /App/Service/Point.Name.01
                        - /App/Service/Point.Name.02
            #service ProfinetClient Ied01:
            #    cycle: 500 ms                         # operating cycle time of the module
            "#,
        ];
        for conf in conf {
            match serde_yaml::from_str(conf) {
                Ok(conf) => {
                    let conf: serde_yaml::Value = conf;
                    let (_key, conf) = conf.as_mapping().unwrap().into_iter().next().unwrap();
                    // let conf = ConfTree::new(key.as_str().unwrap(), conf.clone());
                    let conf_subscribe = ConfSubscribe::new(conf.clone(), vec![]);
                    println!("conf: {:#?}", conf);
                },
                Err(err) => {
                    println!("Deserialize error: {:#?}", err);
                },
            };
        }
            
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }    
}
