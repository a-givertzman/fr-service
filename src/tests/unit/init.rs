use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

pub struct TestSession {}

impl TestSession {
    pub fn init() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                match env_logger::builder().is_test(true).try_init() {
                    Ok(_) => {
                        println!("TestSession.init | Ok\n")
                    },
                    Err(err) => {
                        println!("TestSession.init | error: {:?}", err)
                    },
                };
            }
        )
    }
}
