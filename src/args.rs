use clap::Parser;

/// Defines the arguments and flags the user can input into the CLI tool.
#[derive(Debug, Parser)]
#[command(name = "cctr")]
#[command(version = "1.0")]
#[command(
    about = "The tr utility copies the standard input to the standard output \
    with substitution or deletion of selected characters.", 
    long_about = None
)]
pub struct Cli {

    /// Complement the set of characters in string1, that is “-C ab” includes 
    /// every character except for ‘a’ and ‘b’.
    #[arg(short = 'C')]
    pub complement1: bool,
    
    /// Same as -C but complement the set of values in string1.
    #[arg(short)]
    pub complement2: bool,

    /// Delete characters in string1 from the input.
    #[arg(short)]
    pub delete: bool,

    /// Squeeze multiple occurrences of the characters listed in the last 
    /// operand (either string1 or string2) in the input into a single instance 
    /// of the character. This occurs after all deletion and translation is 
    /// completed.
    #[arg(short)]
    pub squeeze: bool,

    /// Guarantee that any output is unbuffered.
    #[arg(short)]
    pub unbuffered: bool,

    /// A set of characters to translate into the characters in `string2`
    #[arg(value_parser = not_empty)]
    pub string1: String,

    /// A set of characters to replace the characters in `string1`
    #[arg(value_parser = not_empty)]
    pub string2: Option<String>,
}

/// Make sure that the string arguments are not empty
fn not_empty(arg: &str) -> Result<String, String> {
    
    let s: String = arg
        .parse()
        .map_err(|_| format!("`{arg}` is not a string."))?;
    
    if s.is_empty() {
        Err(format!("{s} contains no characters"))
    } else {
        Ok(s as String)
    }
}