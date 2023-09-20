use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author,version,about, long_about = None)]
pub struct NspeedArgs {
    #[command(subcommand)]
    pub speed_test_command: SpeedTestCommand,
}

#[derive(Subcommand, Debug)]
pub enum SpeedTestCommand {
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
