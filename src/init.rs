use crate::args::Cli;
use anyhow::Result;

/// Read the user inputs, check they're valid and create a BufReader
/// for the provided file.
pub fn init(args: &Cli) -> Result<()> {

    println!("complement1: {:?}", args.complement1);
    println!("complement2: {:?}", args.complement2);
    println!("delete: {:?}", args.delete);
    println!("squeeze: {:?}", args.squeeze);
    println!("unbuffered: {:?}", args.unbuffered);
    println!("string1: {:?}", args.string1);
    println!("string2: {:?}", args.string2);

    Ok(())

}