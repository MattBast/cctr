use anyhow::{Result};
use clap::Parser;
use std::process::exit;


fn main() -> Result<()> {
    // Get the args from the command line.
    let args = cctr::args::Cli::parse();

    // Decide what mode to run the application in
    let _mode = cctr::init::init(&args)?;

    exit(exitcode::OK);
}