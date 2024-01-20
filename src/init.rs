use crate::args::Cli;
use anyhow::Result;
use std::process::exit;
use log::error;

/// Defines the different modes that the application can be run in.
pub enum Mode {
    /// Translate the characters from string1 to string2
    Translate,
    /// Delete the characters listed in string1
    Delete,
    /// Remove the duplicate characters listed in string1
    Compress,
    /// Delete the characters listed in string1 and remove
    /// duplicate characters listed in string2
    DeleteCompress,
}

/// Decide what mode to run the application in
pub fn init(args: &Cli) -> Result<Mode> {


    if args.string2 == None && args.delete && !args.squeeze {
        Ok(Mode::Delete)
    }
    else if args.string2 == None && !args.delete && args.squeeze {
        Ok(Mode::Compress)
    }
    else if args.string2 != None && !args.delete {
        Ok(Mode::Translate)
    }
    else if args.string2 != None && args.delete && args.squeeze {
        Ok(Mode::DeleteCompress)
    }
    else {
        error!("usage: 
            tr [-Ccsu] string1 string2
            tr [-Ccu] -d string1
            tr [-Ccu] -s string1
            tr [-Ccu] -ds string1 string2"
        );
        exit(2)
    }

}