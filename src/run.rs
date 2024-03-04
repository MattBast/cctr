use crate::args::Cli;
use crate::init::Mode;

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, IsTerminal, Write};
use std::iter::zip;

/// Translate, delete and/or compress the strings held in `args`. Use `mode` to
/// decide whether to translate, delete and/or compress. Write the output to stdout.
pub fn run(args: &mut Cli, mode: &Mode) -> Result<()> {
    if stdin().is_terminal() {
        // Request a line of text from the user
        let mut line = String::new();
        stdin().read_line(&mut line)?;

        process_line(line, args, mode)?;
    } else {
        // Read the lines of text received from another cli application
        let reader = BufReader::new(stdin().lock());
        reader
            .lines()
            .try_for_each(|line| process_line(line?, args, mode))?;
    }

    Ok(())
}

/// Translate, delete and/or compress a single line
fn process_line(line: String, args: &mut Cli, mode: &Mode) -> Result<()> {
    let mut writer_handle = BufWriter::new(stdout());

    match mode {
        Mode::Translate => translate(line, args, &mut writer_handle),
        Mode::Delete => delete(line, args, &mut writer_handle),
        Mode::Compress => compress(line, args),
        Mode::DeleteCompress => delete_and_compress(line, args),
    }
}

/// Translate the given line using string1 and string2 in the args. Write the translated line
/// to writer.
fn translate(mut line: String, args: &mut Cli, mut writer: impl Write) -> Result<()> {
    // Extract strings from args
    let string1 = &mut args.string1;
    let string2 = &mut args.string2.clone().unwrap();

    // Search for graphemes rather than chars to handle unicode characters like "a̐"
    let mut graphemes1 = get_patterns(string1)?;
    let mut graphemes2 = get_patterns(string2)?;

    // Make sure both strings are the same length. If not, pad the shorter one
    // with whitespace chracters
    if graphemes1.len() > graphemes2.len() {
        graphemes2.resize(graphemes1.len(), graphemes2.last().unwrap().clone());
    } else {
        graphemes1.truncate(graphemes2.len());
    }

    // Replace all chars found in string1 with the chars found in string2
    for (char1, char2) in zip(graphemes1, graphemes2) {
        line = match char1 {
            Pattern::Alnum => translate_alphanumerics(line, char2)?,
            Pattern::Alpha => translate_alphabetic(line, char2)?,
            Pattern::Blank => translate_blank(line, char2)?,
            Pattern::Cntrl => translate_control(line, char2)?,
            Pattern::Digit => translate_digit(line, char2)?,
            Pattern::Lower => translate_lowercase(line, char2)?,
            Pattern::Space => translate_blank(line, char2)?,
            Pattern::Upper => translate_uppercase(line, char2)?,
            Pattern::Char(c) => translate_char(line, c, char2)?,
        }
    }

    writeln!(writer, "{}", line).with_context(|| "Unable to write line to writer.".to_string())
}

/// Defines the patterns in string1 and string2 to process
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Represents a grapheme (character)
    Char(char),
    Alnum,
    Alpha,
    Blank,
    Cntrl,
    Digit,
    Lower,
    Space,
    Upper,
}

/// Extract graphemes (characters) and classes ready to translate a string by
fn get_patterns(string: &mut str) -> Result<Vec<Pattern>> {
    // match patterns that are either words flanked by [::]
    // or match against single characters
    let re = Regex::new(r"\[:([^:]+):]|.")?;
    let str_patterns: Vec<&str> = re.find_iter(string).map(|m| m.as_str()).collect();

    // start a vector to hold the patterns in
    let mut patterns = Vec::new();

    // loop through the patterns and parse each pattern into a Pattern type
    for str_pattern in str_patterns {
        if str_pattern.len() > 1 {
            match str_pattern {
                "[:alnum:]" => patterns.push(Pattern::Alnum),
                "[:alpha:]" => patterns.push(Pattern::Alpha),
                "[:blank:]" => patterns.push(Pattern::Blank),
                "[:cntrl:]" => patterns.push(Pattern::Cntrl),
                "[:digit:]" => patterns.push(Pattern::Digit),
                "[:lower:]" => patterns.push(Pattern::Lower),
                "[:space:]" => patterns.push(Pattern::Space),
                "[:upper:]" => patterns.push(Pattern::Upper),
                _ => return Err(anyhow!("Invalid class.")),
            }
        } else {
            patterns.push(Pattern::Char(str_pattern.chars().next().unwrap()));
        }
    }

    Ok(patterns)
}

/// Translate the alphanumeric characters
fn translate_alphanumerics(mut line: String, pattern: Pattern) -> Result<String> {
    line = match pattern {
        Pattern::Alnum => line,
        Pattern::Alpha => line
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    char::from_u32((c as u32) + 10).unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Blank => line
            .chars()
            .map(|c| if c.is_alphanumeric() { ' ' } else { c })
            .collect(),
        Pattern::Cntrl => line
            .chars()
            .map(|c| if c.is_alphanumeric() { ' ' } else { c })
            .collect(),
        Pattern::Digit => line
            .chars()
            .map(|c| {
                if c.is_alphanumeric() & !c.is_ascii_digit() {
                    '9'
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Lower => line
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c.to_lowercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Space => line
            .chars()
            .map(|c| if c.is_alphanumeric() { ' ' } else { c })
            .collect(),
        Pattern::Upper => line
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Char(new_c) => line
            .chars()
            .map(|c| if c.is_alphanumeric() { new_c } else { c })
            .collect(),
    };

    Ok(line)
}

/// Translate the alphabetic characters
fn translate_alphabetic(mut line: String, pattern: Pattern) -> Result<String> {
    line = match pattern {
        Pattern::Alnum => line
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    char::from_u32((c as u32) + 10).unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Alpha => line,
        Pattern::Blank => line
            .chars()
            .map(|c| if c.is_alphabetic() { ' ' } else { c })
            .collect(),
        Pattern::Cntrl => line
            .chars()
            .map(|c| if c.is_alphabetic() { ' ' } else { c })
            .collect(),
        Pattern::Digit => line
            .chars()
            .map(|c| {
                if c.is_alphabetic() & !c.is_ascii_digit() {
                    '9'
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Lower => line
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    c.to_lowercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Space => line
            .chars()
            .map(|c| if c.is_alphabetic() { ' ' } else { c })
            .collect(),
        Pattern::Upper => line
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Char(new_c) => line
            .chars()
            .map(|c| if c.is_alphabetic() { new_c } else { c })
            .collect(),
    };

    Ok(line)
}

/// Translate whitespace characters (' ') into the gven pattern
fn translate_blank(mut line: String, pattern: Pattern) -> Result<String> {
    line = match pattern {
        Pattern::Alnum => line
            .chars()
            .map(|c| if c == ' ' { '2' } else { c })
            .collect(),
        Pattern::Alpha => line
            .chars()
            .map(|c| if c == ' ' { 'C' } else { c })
            .collect(),
        Pattern::Blank => line,
        Pattern::Cntrl => line
            .chars()
            .map(|c| if c == ' ' { ' ' } else { c })
            .collect(),
        Pattern::Digit => line
            .chars()
            .map(|c| if c == ' ' { '2' } else { c })
            .collect(),
        Pattern::Lower => line
            .chars()
            .map(|c| {
                if c == ' ' {
                    c.to_lowercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Space => line,
        Pattern::Upper => line
            .chars()
            .map(|c| {
                if c == ' ' {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Char(new_c) => line.replace(' ', &new_c.to_string()),
    };

    Ok(line)
}

/// Translate the control characters
fn translate_control(mut line: String, pattern: Pattern) -> Result<String> {
    line = match pattern {
        Pattern::Alnum => line
            .chars()
            .map(|c| if c.is_control() { 'A' } else { c })
            .collect(),
        Pattern::Alpha => line
            .chars()
            .map(|c| if c.is_control() { 'K' } else { c })
            .collect(),
        Pattern::Blank => line
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect(),
        Pattern::Cntrl => line,
        Pattern::Digit => line
            .chars()
            .map(|c| {
                if c.is_control() & !c.is_ascii_digit() {
                    '9'
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Lower => line
            .chars()
            .map(|c| {
                if c.is_control() {
                    c.to_lowercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Space => line
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect(),
        Pattern::Upper => line
            .chars()
            .map(|c| {
                if c.is_control() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Char(new_c) => line
            .chars()
            .map(|c| if c.is_control() { new_c } else { c })
            .collect(),
    };

    Ok(line)
}

/// Translate the digits into the given pattern
fn translate_digit(mut line: String, pattern: Pattern) -> Result<String> {
    line = match pattern {
        Pattern::Alnum => line
            .chars()
            .map(|c| {
                if c.is_numeric() {
                    char::from_u32(c as u32).unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Alpha => line
            .chars()
            .map(|c| {
                if c.is_numeric() {
                    char::from_u32(c as u32).unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Blank => line
            .chars()
            .map(|c| if c.is_numeric() { ' ' } else { c })
            .collect(),
        Pattern::Cntrl => line
            .chars()
            .map(|c| if c.is_numeric() { ' ' } else { c })
            .collect(),
        Pattern::Digit => line,
        Pattern::Lower => line
            .chars()
            .map(|c| {
                if c.is_numeric() {
                    char::from_u32((c.to_digit(10).unwrap() as u32) + 97).unwrap().to_lowercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Space => line
            .chars()
            .map(|c| if c.is_numeric() { ' ' } else { c })
            .collect(),
        Pattern::Upper => line
            .chars()
            .map(|c| {
                if c.is_numeric() {
                    char::from_u32((c.to_digit(10).unwrap() as u32) + 65).unwrap().to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Char(new_c) => line
            .chars()
            .map(|c| if c.is_numeric() { new_c } else { c })
            .collect(),
    };

    Ok(line)
}

/// Translate the lowercase characters into the given pattern
fn translate_lowercase(mut line: String, pattern: Pattern) -> Result<String> {
    line = match pattern {
        Pattern::Alnum => line
            .chars()
            .map(|c| {
                if c.is_lowercase() {
                    char::from_u32((c as u32) - 49).unwrap()
                }
                else {
                    c
                }
            })
            .collect(),
        Pattern::Alpha => line
            .chars()
            .map(|c| {
                if c.is_lowercase() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Blank => line
            .chars()
            .map(|c| if c.is_lowercase() { ' ' } else { c })
            .collect(),
        Pattern::Cntrl => line
            .chars()
            .map(|c| if c.is_lowercase() { ' ' } else { c })
            .collect(),
        Pattern::Digit => line
            .chars()
            .map(|c| {
                if c.is_lowercase() {
                    if c as u32 <= 48 {
                        char::from_u32((c as u32) - 49).unwrap()    
                    }
                    else {
                        '9'
                    }
                }
                else {
                    c
                }
            })
            .collect(),
        Pattern::Lower => line,
        Pattern::Space => line
            .chars()
            .map(|c| if c.is_lowercase() { ' ' } else { c })
            .collect(),
        Pattern::Upper => line
            .chars()
            .map(|c| {
                if c.is_lowercase() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Char(new_c) => line
            .chars()
            .map(|c| if c.is_lowercase() { new_c } else { c })
            .collect(),
    };

    Ok(line)
}

/// Translate the lowercase characters into the given pattern
fn translate_uppercase(mut line: String, pattern: Pattern) -> Result<String> {
    line = match pattern {
        Pattern::Alnum => line
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    char::from_u32((c as u32) - 49).unwrap()
                }
                else {
                    c
                }
            })
            .collect(),
        Pattern::Alpha => line
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Blank => line
            .chars()
            .map(|c| if c.is_uppercase() { ' ' } else { c })
            .collect(),
        Pattern::Cntrl => line
            .chars()
            .map(|c| if c.is_uppercase() { ' ' } else { c })
            .collect(),
        Pattern::Digit => line
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    if c as u32 <= 48 {
                        char::from_u32((c as u32) - 49).unwrap()    
                    }
                    else {
                        '9'
                    }
                }
                else {
                    c
                }
            })
            .collect(),
        Pattern::Lower => line
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Space => line
            .chars()
            .map(|c| if c.is_uppercase() { ' ' } else { c })
            .collect(),
        Pattern::Upper => line,
        Pattern::Char(new_c) => line
            .chars()
            .map(|c| if c.is_uppercase() { new_c } else { c })
            .collect(),
    };

    Ok(line)
}

/// Translate a character (pattern1) into pattern2
fn translate_char(mut line: String, pattern1: char, pattern2: Pattern) -> Result<String> {
    line = match pattern2 {
        Pattern::Alnum => line
            .chars()
            .map(|c| if c == pattern1 { '0' } else { c })
            .collect(),
        Pattern::Alpha => line
            .chars()
            .map(|c| if c == pattern1 { 'A' } else { c })
            .collect(),
        Pattern::Blank => line
            .chars()
            .map(|c| if c == pattern1 { ' ' } else { c })
            .collect(),
        Pattern::Cntrl => line
            .chars()
            .map(|c| if c == pattern1 { ' ' } else { c })
            .collect(),
        Pattern::Digit => line
            .chars()
            .map(|c| if c == pattern1 { '0' } else { c })
            .collect(),
        Pattern::Lower => line
            .chars()
            .map(|c| {
                if c == pattern1 {
                    c.to_lowercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Space => line
            .chars()
            .map(|c| if c == pattern1 { ' ' } else { c })
            .collect(),
        Pattern::Upper => line
            .chars()
            .map(|c| {
                if c == pattern1 {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                }
            })
            .collect(),
        Pattern::Char(new_c) => line.replace(pattern1, &new_c.to_string()),
    };

    Ok(line)
}

fn delete(mut line: String, args: &mut Cli, mut writer: impl Write) -> Result<()> {

    // Extract the only string we need from from args and extract a vector 
    // of patterns to delete from it
    let string1 = &mut args.string1;
    let patterns = get_patterns(string1)?;

    // Replace all chars found in string1 with the chars found in string2
    for pattern in patterns {
        line = match pattern {
            Pattern::Alnum => line.replace(char::is_alphanumeric, ""),
            Pattern::Alpha => line.replace(char::is_alphabetic, ""),
            Pattern::Blank => line.replace(char::is_whitespace, ""),
            Pattern::Cntrl => line.replace(char::is_control, ""),
            Pattern::Digit => line.replace(char::is_numeric, ""),
            Pattern::Lower => line.replace(char::is_lowercase, ""),
            Pattern::Space => line.replace(char::is_whitespace, ""),
            Pattern::Upper => line.replace(char::is_uppercase, ""),
            Pattern::Char(c) => line.replace(c, ""),
        }
    }

    writeln!(writer, "{}", line).with_context(|| "Unable to write line to writer.".to_string())
    
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
    fn can_translate_single_letters() {
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
    fn can_translate_letter_to_number() {
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
    fn can_translate_empty_line() {
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
    fn can_translate_many_letters() {
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
    fn string1_can_be_longer_than_string2() {
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
    fn string2_can_be_longer_than_string1() {
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
    fn can_translate_single_letters_across_multiple_lines() {
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
    fn string1_can_be_longer_than_string2_with_two_lines() {
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
    fn can_translate_special_characters() {
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
    fn can_translate_lower_to_upper_class() {
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

    #[test]
    fn can_translate_upper_to_lower_class() {
        let line = "CODING CHALLENGE".to_string();

        let mut args = Cli {
            string1: "[:upper:]".to_string(),
            string2: Some("[:lower:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"coding challenge\n");
    }

    #[test]
    fn can_translate_space_class() {
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "[:space:]".to_string(),
            string2: Some("_".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"coding_challenge\n");
    }

    #[test]
    fn can_translate_blank_class() {
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "[:blank:]".to_string(),
            string2: Some("_".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"coding_challenge\n");
    }

    #[test]
    fn can_translate_alphanumeric_class() {
        let line = "123_challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("a".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"aaa_aaaaaaaaa\n");
    }

    #[test]
    fn can_translate_alphabetic_class() {
        let line = "123_challenge".to_string();

        let mut args = Cli {
            string1: "[:alpha:]".to_string(),
            string2: Some("a".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"123_aaaaaaaaa\n");
    }

    #[test]
    fn can_translate_control_class() {
        let line = "123\tchallenge".to_string();

        let mut args = Cli {
            string1: "[:cntrl:]".to_string(),
            string2: Some(" ".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"123 challenge\n");
    }

    #[test]
    fn can_translate_digit_class() {
        let line = "123 challenge".to_string();

        let mut args = Cli {
            string1: "[:digit:]".to_string(),
            string2: Some("a".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"aaa challenge\n");
    }

    #[test]
    fn can_translate_alphanumeric_to_alphabetic() {
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("[:alpha:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"mynsxq mrkvvoxqo\n");
    }

    #[test]
    fn can_translate_alphanumeric_to_blank() {
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("[:blank:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"                \n");
    }

    #[test]
    fn can_translate_alphanumeric_to_control_character() {
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("[:cntrl:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"                \n");
    }

    #[test]
    fn can_translate_alphanumeric_to_digit() {
        let line = "1oding challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("[:digit:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"199999 999999999\n");
    }

    #[test]
    fn can_translate_alphanumeric_to_lower() {
        let line = "Coding challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("[:lower:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"coding challenge\n");
    }

    #[test]
    fn can_translate_alphanumeric_to_space() {
        let line = "coding challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("[:space:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"                \n");
    }

    #[test]
    fn can_translate_alphanumeric_to_upper() {
        let line = "Coding challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            string2: Some("[:upper:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"CODING CHALLENGE\n");
    }

    #[test]
    fn can_translate_digits_to_chars() {
        let line = "01234 challenge".to_string();

        let mut args = Cli {
            string1: "[:digit:]".to_string(),
            string2: Some("a".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"aaaaa challenge\n");
    }

    #[test]
    fn can_translate_digits_to_control_chars() {
        let line = "01234 challenge".to_string();

        let mut args = Cli {
            string1: "[:digit:]".to_string(),
            string2: Some("[:cntrl:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"      challenge\n");
    }

    #[test]
    fn can_translate_digits_to_lowercase_chars() {
        let line = "01234 challenge".to_string();

        let mut args = Cli {
            string1: "[:digit:]".to_string(),
            string2: Some("[:lower:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"abcde challenge\n");
    }

    #[test]
    fn can_translate_digits_to_uppercase_chars() {
        let line = "01234 challenge".to_string();

        let mut args = Cli {
            string1: "[:digit:]".to_string(),
            string2: Some("[:upper:]".to_string()),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = translate(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"ABCDE challenge\n");
    }

    // ************************************************************************
    // delete tests
    // ************************************************************************
    #[test]
    fn can_delete_single_characters() {
        let line = "Coding challenge".to_string();

        let mut args = Cli {
            string1: "C".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"oding challenge\n");
    }

    #[test]
    fn can_delete_many_characters() {
        let line = "Coding challenge".to_string();

        let mut args = Cli {
            string1: "Cdg".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"oin challene\n");
    }

    #[test]
    fn can_delete_nothing() {
        let line = "Coding challenge".to_string();

        let mut args = Cli {
            string1: "".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"Coding challenge\n");
    }

    #[test]
    fn can_delete_everything() {
        let line = "Coding challenge".to_string();

        let mut args = Cli {
            string1: "Coding challenge".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"\n");
    }

    #[test]
    fn can_delete_digits() {
        let line = "123 challenge".to_string();

        let mut args = Cli {
            string1: "123".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b" challenge\n");
    }

    #[test]
    fn can_delete_whitespace() {
        let line = "Coding challenge".to_string();

        let mut args = Cli {
            string1: " ".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"Codingchallenge\n");
    }

    #[test]
    fn can_delete_special_chars() {
        let line = "Coding@challenge".to_string();

        let mut args = Cli {
            string1: "@".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"Codingchallenge\n");
    }

    #[test]
    fn can_delete_upper_class() {
        let line = "CoDinG challenge".to_string();

        let mut args = Cli {
            string1: "[:upper:]".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"oin challenge\n");
    }

    #[test]
    fn can_delete_lower_class() {
        let line = "CoDinG challenge".to_string();

        let mut args = Cli {
            string1: "[:lower:]".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"CDG \n");
    }

    #[test]
    fn can_delete_alphabetic_chars() {
        let line = "123 challenge".to_string();

        let mut args = Cli {
            string1: "[:alpha:]".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"123 \n");
    }

    #[test]
    fn can_delete_alphanumeric_chars() {
        let line = "123@challenge".to_string();

        let mut args = Cli {
            string1: "[:alnum:]".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"@\n");
    }

    #[test]
    fn can_delete_blanks() {
        let line = "123 challenge".to_string();

        let mut args = Cli {
            string1: "[:blank:]".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"123challenge\n");
    }

    #[test]
    fn can_delete_control_characters() {
        let line = "123\tchallenge".to_string();

        let mut args = Cli {
            string1: "[:cntrl:]".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b"123challenge\n");
    }

    #[test]
    fn can_delete_digit_characters() {
        let line = "٣7৬¾ challenge".to_string();

        let mut args = Cli {
            string1: "[:digit:]".to_string(),
            ..Default::default()
        };

        let mut writer = Vec::new();

        let result = delete(line, &mut args, &mut writer);

        assert!(result.is_ok());
        assert_eq!(writer, b" challenge\n");
    }

    // ************************************************************************
    // translate tests (Ccu flags)
    // ************************************************************************
}
