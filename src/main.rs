use anyhow::{Result};
use clap::Parser;
use std::process::exit;
use log::error;


fn main() -> Result<()> {
    // Get the args from the command line.
    let args = cctr::args::Cli::parse();

    // Decide what mode to run the application in
    let _mode = match cctr::init::init(&args) {
        Ok(mode) => mode,
        Err(e) => {
            error!("{:?}", e);
            exit(2)
        }
    };

    exit(exitcode::OK);
}