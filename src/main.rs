mod cli;
mod client;
mod common;
mod server;

use clap::Parser;
use cli::{args::SpeedTestCommand, NspeedArgs};
use std::env;
use tokio::io;

use crate::{client::nspeed_client, server::nspeed_server};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let args = NspeedArgs::parse();

    match args.speed_test_command {
        SpeedTestCommand::Server { bind, port } => {
            nspeed_server::server(&bind, port).await?;
        }
        SpeedTestCommand::Client {
            host,
            port,
            data,
            iterations,
            format,
            output,
        } => {
            nspeed_client::client(&host, port, data, iterations, format, output).await?;
        }
    }

    Ok(())
}
