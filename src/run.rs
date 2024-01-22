use crate::args::Cli;
use crate::init::Mode;

use anyhow::Result;
use std::io::{stdin, BufRead, BufReader, IsTerminal};

/// Translate, delete and/or compress the strings held in `args`. Use `mode` to
/// decide whether to translate, delete and/or compress. Write the output to stdout.
pub fn run(args: &Cli, mode: &Mode) -> Result<()> {

	if stdin().is_terminal() {
		
		// Request a line of text from the user
		let mut line = String::new();
	    stdin().read_line(&mut line)?;

	    process_line(line, args, mode)?;

	}
	else {
		
		// Read the lines of text received from another cli application
		let reader = BufReader::new(stdin().lock());
		reader
			.lines()
			.try_for_each(|line| {
				process_line(line?, args, mode)
			})?;

	}
	
	Ok(())

}

/// Translate, delete and/or compress a single line
fn process_line(line: String, args: &Cli, mode: &Mode) -> Result<()> {
	
	match mode {
		Mode::Translate => translate(line, args),
		Mode::Delete => delete(line, args),
		Mode::Compress => compress(line, args),
		Mode::DeleteCompress => delete_and_compress(line, args)
	}

}


fn translate(line: String, args: &Cli) -> Result<()> {
	println!("Translate");
	println!("{:?}", line);
	println!("{:?}", args);
	Ok(())
}


fn delete(line: String, args: &Cli) -> Result<()> {
	println!("Delete");
	println!("{:?}", line);
	println!("{:?}", args);
	Ok(())
}


fn compress(line: String, args: &Cli) -> Result<()> {
	println!("Compress");
	println!("{:?}", line);
	println!("{:?}", args);
	Ok(())
}


fn delete_and_compress(line: String, args: &Cli) -> Result<()> {
	println!("Delete and Compress");
	println!("{:?}", line);
	println!("{:?}", args);
	Ok(())
}