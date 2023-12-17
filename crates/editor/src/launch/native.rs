mod cli;
mod server;

#[cfg(feature = "bundled")]
mod bundle;

use self::cli::{Command, Options};
use structopt::StructOpt;

pub async fn launch() -> Result<(), eframe::Error> {
    env_logger::init();
    let Options { command } = Options::from_args();
    start_editor(command).await
}

async fn start_editor(command: Command) -> Result<(), eframe::Error> {
    match command {
        Command::Server { port } => server::listen(port).await,
        Command::Desktop => return render_native_ui(),

        #[cfg(feature = "bundled")]
        Command::Browser { address, port } => bundle::launch_browser_ui(&address, port).await,
    };
    Ok(())
}

fn render_native_ui() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My app name",
        native_options,
        Box::new(|cc| Box::new(crate::app::App::new(cc))),
    )
}
