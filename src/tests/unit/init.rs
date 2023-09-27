use std::env;
use std::sync::Once;

use env_logger::Builder;

static INIT: Once = Once::new();

pub fn tryInit() {
    INIT.call_once(|| {
            env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
            // env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_BACKTRACE", "full");
            let mut builder = Builder::new();
            builder.try_init();
            // env_logger::init();
        }
    )
}
