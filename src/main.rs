use std::env;
use clap::Parser;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use log::error;
use services::app::app::App;
use crate::core_::cli::cli::Cli;

#[cfg(test)]
mod tests;
mod core_;
mod conf;
mod services;
mod tcp;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let cli = Cli::parse();
    let path = env::current_dir().unwrap();
    println!("main | working path: \n\t{:?}", path);
    let path = path.join(cli.config.unwrap_or("config.yaml".to_owned()));
    let app = App::new(path);
    if let Err(err) = app.run() {
        error!("main | Error: {:#?}", err);
    };
}
