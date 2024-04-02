use std::env;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use log::error;
use services::app::app::App;

#[cfg(test)]
mod tests;
mod core_;
mod conf;
mod services;
mod tcp;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    // let path = env::current_dir().unwrap();
    println!("main | working path: \n\t{:?}", env::current_dir().unwrap());
    let path = "config.yaml";
    let app = App::new(path);
    if let Err(err) = app.run() {
        error!("main | Error: {:#?}", err);
    };
}
