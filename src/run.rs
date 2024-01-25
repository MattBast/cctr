use crate::args::Cli;
use crate::init::Mode;

use anyhow::{Result, Context};
use std::io::{stdin, BufRead, BufReader, IsTerminal, stdout, Write, BufWriter};
use std::iter::zip;
use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;

/// Translate, delete and/or compress the strings held in `args`. Use `mode` to
/// decide whether to translate, delete and/or compress. Write the output to stdout.
pub fn run(args: &mut Cli, mode: &Mode) -> Result<()> {

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
fn process_line(line: String, args: &mut Cli, mode: &Mode) -> Result<()> {
	
	let mut writer_handle = BufWriter::new(stdout());

	match mode {
		Mode::Translate => translate(line, args, &mut writer_handle),
		Mode::Delete => delete(line, args),
		Mode::Compress => compress(line, args),
		Mode::DeleteCompress => delete_and_compress(line, args)
	}

}


fn translate(mut line: String, args: &mut Cli, mut writer: impl Write) -> Result<()> {
	
	// Extract strings from args
	let string1 = &mut args.string1;
	let string2 = &mut args.string2.clone().unwrap();

	// Make sure both strings are the same length. If not, pad the shorter one
	// with whitespace chracters
	string1.extend(vec![""; string2.len()]);
	string2.extend(vec![string2.chars().last().unwrap(); string1.len()]);

	// Search for graphemes rather than chars to handle unicode characters like "a̐"
	let graphemes1 = string1.graphemes(true);
	let graphemes2 = string2.graphemes(true);

	// Replace all chars found in string1 with the chars found in string2
	for (char1, char2) in zip(graphemes1, graphemes2) {
		let re = Regex::new(char1).unwrap();
		line = re.replace_all(&line, char2).to_string();
		// line = line.replace(char1, char2);
	}

	writeln!(writer, "{}", line)
			.with_context(|| "Unable to write line to writer.".to_string())

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

#[cfg(test)]
mod tests {
    use super::*;

    // ************************************************************************
    // translate tests (characters)
    // ************************************************************************

    #[test]
    fn can_translate_single_letters(){
        
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "c".to_string(),
            string2: Some("C".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"Coding Challenge\n");
        
    }

    #[test]
    fn can_translate_letter_to_number(){
        
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "c".to_string(),
            string2: Some("3".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"3oding 3hallenge\n");
        
    }

    #[test]
    fn can_translate_empty_line(){
        
        let line = "".to_string();

        let mut args = Cli {
            string1: "c".to_string(),
            string2: Some("C".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"\n");
        
    }

    #[test]
    fn can_translate_many_letters(){
        
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "cod".to_string(),
            string2: Some("COD".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"CODing Challenge\n");
        
    }

    #[test]
    fn string1_can_be_longer_than_string2(){
        
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "cod".to_string(),
            string2: Some("C".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"CCCing Challenge\n");
        
    }

    #[test]
    fn string2_can_be_longer_than_string1(){
        
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "c".to_string(),
            string2: Some("COD".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"Coding Challenge\n");
        
    }

    #[test]
    fn can_translate_single_letters_across_multiple_lines(){
        
        let line1 = "coding challenge".to_string();
        let line2 = "abcabcabc".to_string();
        let line3 = "come as you are".to_string();

        let mut args = Cli {
            string1: "c".to_string(),
            string2: Some("C".to_string()),
            ..Default::default()
        };

        // test first line
        let mut writer = Vec::new();
        let result = translate(line1, &mut args, &mut writer);
        assert!(result.is_ok());
        assert_eq!(writer, b"Coding Challenge\n");

        // test second line
        writer.clear();
        let result = translate(line2, &mut args, &mut writer);
        assert!(result.is_ok());
        assert_eq!(writer, b"abCabCabC\n");

        // test second line
        writer.clear();
        let result = translate(line3, &mut args, &mut writer);
        assert!(result.is_ok());
        assert_eq!(writer, b"Come as you are\n");
        
    }

    #[test]
    fn string1_can_be_longer_than_string2_with_two_lines(){
        
        let line1 = "coding challenge".to_string();
        let line2 = "coding challenge".to_string();

        let mut args = Cli {
            string1: "cod".to_string(),
            string2: Some("C".to_string()),
            ..Default::default()
        };

        // test first line
        let mut writer = Vec::new();
        let result = translate(line1, &mut args, &mut writer);
        assert!(result.is_ok());
        assert_eq!(writer, b"CCCing Challenge\n");

        // test second line
        writer.clear();
        let result = translate(line2, &mut args, &mut writer);
        assert!(result.is_ok());
        assert_eq!(writer, b"CCCing Challenge\n");
        
    }

    #[test]
    fn can_translate_special_characters(){
        
        let line = "{coding challenge}".to_string();

        let mut args = Cli {
            string1: "{}".to_string(),
            string2: Some("[]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"[coding challenge]\n");
        
    }

    // ************************************************************************
    // translate tests (classes)
    // ************************************************************************

     #[test]
    fn can_translate_upper_class(){
        
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "[:lower:]".to_string(),
            string2: Some("[:upper:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"CODING CHALLENGE\n");
        
    }

}