use bwp::{app, cli::Cli, errors};
use clap::Parser;
use color_eyre::Result;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    errors::init()?;
    bwp::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}
