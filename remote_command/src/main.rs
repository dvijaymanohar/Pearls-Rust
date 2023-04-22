pub mod remotecli;

use structopt::StructOpt;

// These are the options used by the `server` subcommand
#[derive(Debug, StructOpt)] // to let the compiler know to generate the command line parser.
pub struct ServerOptions {
    /// The address of the server that will run commands.
    #[structopt(long, default_value = "127.0.0.1:50051")]
    pub server_listen_addr: String,
}

// These are the options used by the `run` subcommand
#[derive(Debug, StructOpt)]
pub struct RemoteCommandOptions {
    /// The address of the server that will run commands.
    #[structopt(long = "server", default_value = "http://127.0.0.1:50051")]
    pub server_addr: String,

    /// The full command and arguments for the server to execute
    #[structopt(long = "command", default_value = "ls -al")]
    pub command: Vec<String>,
}

// These are the only valid values for our subcommands
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Start the remote command gRPC server
    #[structopt(name = "server")]
    StartServer(ServerOptions),

    /// Send a remote command to the gRPC server
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Run(RemoteCommandOptions),
}

// This is the main arguments structure that we'll parse from
#[derive(StructOpt, Debug)]
#[structopt(name = "remotecli")]
struct ApplicationArguments {
    #[structopt(flatten)]
    pub subcommand: SubCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ApplicationArguments::from_args();

    // our CLI to switch between running as a client or running as a server.
    match args.subcommand {
        SubCommand::StartServer(opts) => {
            println!("Start the server on: {:?}", opts.server_listen_addr);
            remotecli::server::start_server(opts).await?;
        }

        SubCommand::Run(rc_opts) => {
            println!("Run command: '{:?}'", rc_opts.command);
            println!("Start the server on: {:?}", rc_opts.server_addr);
            remotecli::client::client_run(rc_opts).await?;
        }
    }

    Ok(())
}

// Server:
// cargo run -- server --server-listen-addr 131.180.32.250:60054
// cargo run -- server // Local host

// Client:

// cargo run -- run ls -al // Local host
// cargo run -- run --server "http://131.180.32.250:45353" --command "ls"
