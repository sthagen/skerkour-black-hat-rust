use std::time::Duration;

mod commands;
mod config;
mod error;
mod init;
mod install;
mod run;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("ch12_agent/0.1")
        .build();

    let conf = init::init(&api_client)?;
    run::run(&api_client, conf);
}
