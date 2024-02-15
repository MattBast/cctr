use anyhow::Result;
use clap::Parser;
use log::error;
use std::process::exit;

fn main() -> Result<()> {
    // Get the args from the command line.
    let mut args = cctr::args::Cli::parse();

    // Decide what mode to run the application in
    let mode = match cctr::init::init(&args) {
        Ok(mode) => mode,
        Err(e) => {
            error!("{:?}", e);
            exit(2)
        }
    };

    cctr::run::run(&mut args, &mode)?;

    exit(exitcode::OK);
}
