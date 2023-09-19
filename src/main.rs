mod client;
mod server;

use clap::{arg, command, Parser, Subcommand};
use std::env;
use tokio::io;

use crate::{client::nspeed_client, server::nspeed_server};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
#[command(author,version,about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    speed_test_command: SpeedTestCommand,
}

#[derive(Subcommand, Debug)]
enum SpeedTestCommand {
    Server {
        #[arg(short, long, default_value_t = String::from("0.0.0.0"), help = "Binding adress for server")]
        bind: String,

        #[arg(short, long, default_value_t = 6666, help = "Server port")]
        port: usize,
    },

    Client {
        #[arg(short = 'H', long, default_value_t = String::from("0.0.0.0"), help = "Adress for server")]
        host: String,

        #[arg(short, long, default_value_t = 6666, help = "Server port")]
        port: usize,

        #[arg(
            short,
            long,
            default_value_t = 800,
            help = "Amount of data to be sent/received under test"
        )]
        data: usize,
    },
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
    let args = Cli::parse();

    match args.speed_test_command {
        SpeedTestCommand::Server { bind, port } => {
            nspeed_server::server(&bind, port).await?;
        }
        SpeedTestCommand::Client { host, port, data } => {
            nspeed_client::client(&host, port, data).await?;
        }
    }

    Ok(())
}
