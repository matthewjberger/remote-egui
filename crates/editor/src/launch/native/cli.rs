use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "My app name", about = "🛠 my app description 🛠")]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Launches the standalone desktop client.
    #[structopt(about = "Launch a standalone desktop client")]
    Desktop,

    /// Launches the application in a web browser.
    #[cfg(feature = "bundled")]
    #[structopt(about = "Launch a browser client")]
    Browser {
        /// The address to serve
        #[structopt(
            short,
            long,
            default_value = "localhost",
            about = "The address to serve"
        )]
        address: String,

        /// The port for the server to listen on
        #[structopt(
            short,
            long,
            default_value = "9002",
            about = "The port the server will listen on"
        )]
        port: u16,
    },

    /// Starts the server to allow remote client connections.
    #[structopt(about = "Launch a server to accept connections from remote clients")]
    Server {
        /// The port for the server to listen on
        #[structopt(
            short,
            long,
            default_value = "9000",
            about = "The port the server will listen on"
        )]
        port: u16,
    },
}
