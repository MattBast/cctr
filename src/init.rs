use crate::args::Cli;
use anyhow::{Result, anyhow};

/// Defines the different modes that the application can be run in.
#[derive(Debug)]
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
        Err(anyhow!("usage: 
            tr [-Ccsu] string1 string2
            tr [-Ccu] -d string1
            tr [-Ccu] -s string1
            tr [-Ccu] -ds string1 string2"
        ))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // ************************************************************************
    // translate mode tests
    // ************************************************************************

    #[test]
    fn can_run_in_translate_mode(){
        
        let args = Cli {
            complement1: false,
            complement2: false,
            delete: false,
            squeeze: false,
            unbuffered: false,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Translate));
        
    }

    #[test]
    fn can_use_the_capital_c_flag_in_translate_mode(){
        
        let args = Cli {
            complement1: true,
            complement2: false,
            delete: false,
            squeeze: false,
            unbuffered: false,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Translate));
        
    }

    #[test]
    fn can_use_the_c_flag_in_translate_mode(){
        
        let args = Cli {
            complement1: false,
            complement2: true,
            delete: false,
            squeeze: false,
            unbuffered: false,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Translate));
        
    }

    #[test]
    fn can_use_the_s_flag_in_translate_mode(){
        
        let args = Cli {
            complement1: false,
            complement2: false,
            delete: false,
            squeeze: true,
            unbuffered: false,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Translate));
        
    }

    #[test]
    fn can_use_the_u_flag_in_translate_mode(){
        
        let args = Cli {
            complement1: false,
            complement2: false,
            delete: false,
            squeeze: false,
            unbuffered: true,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Translate));
    }

    #[test]
    fn can_use_the_ccsu_flags_in_translate_mode(){
        
        let args = Cli {
            complement1: true,
            complement2: true,
            delete: false,
            squeeze: true,
            unbuffered: true,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Translate));
        
    }

    // ************************************************************************
    // delete mode tests
    // ************************************************************************

    #[test]
    fn can_run_in_delete_mode(){
        
        let args = Cli {
            complement1: false,
            complement2: false,
            delete: true,
            squeeze: false,
            unbuffered: false,
            string1: "c".to_string(),
            string2: None
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Delete));
        
    }

    #[test]
    fn can_run_in_delete_mode_with_ccu_flags(){
        
        let args = Cli {
            complement1: true,
            complement2: true,
            delete: true,
            squeeze: false,
            unbuffered: true,
            string1: "c".to_string(),
            string2: None
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Delete));
        
    }

    #[test]
    fn two_string_with_just_the_delete_flag_return_error(){
        
        let args = Cli {
            complement1: false,
            complement2: false,
            delete: true,
            squeeze: false,
            unbuffered: false,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let res = init(&args);

        assert!(res.is_err());

        let error = res.unwrap_err();
        let root_cause = error.root_cause();
        assert_eq!(format!("{}", root_cause), "usage: 
            tr [-Ccsu] string1 string2
            tr [-Ccu] -d string1
            tr [-Ccu] -s string1
            tr [-Ccu] -ds string1 string2"
        );
        
    }


    #[test]
    fn one_string_and_no_d_or_s_flag_returns_error(){
        
        let args = Cli {
            complement1: false,
            complement2: false,
            delete: false,
            squeeze: false,
            unbuffered: false,
            string1: "c".to_string(),
            string2: None
        };

        let res = init(&args);

        assert!(res.is_err());

        let error = res.unwrap_err();
        let root_cause = error.root_cause();
        assert_eq!(format!("{}", root_cause), "usage: 
            tr [-Ccsu] string1 string2
            tr [-Ccu] -d string1
            tr [-Ccu] -s string1
            tr [-Ccu] -ds string1 string2"
        );
        
    }

    #[test]
    fn one_string_and_flag_that_is_not_d_or_s_returns_error(){
        
        let args = Cli {
            complement1: true,
            complement2: false,
            delete: false,
            squeeze: false,
            unbuffered: false,
            string1: "c".to_string(),
            string2: None
        };

        let res = init(&args);

        assert!(res.is_err());

        let error = res.unwrap_err();
        let root_cause = error.root_cause();
        assert_eq!(format!("{}", root_cause), "usage: 
            tr [-Ccsu] string1 string2
            tr [-Ccu] -d string1
            tr [-Ccu] -s string1
            tr [-Ccu] -ds string1 string2"
        );
        
    }

    #[test]
    fn adding_all_flags_and_only_one_string_returns_error(){
        
        let args = Cli {
            complement1: true,
            complement2: true,
            delete: true,
            squeeze: true,
            unbuffered: true,
            string1: "c".to_string(),
            string2: None
        };

        let res = init(&args);

        assert!(res.is_err());

        let error = res.unwrap_err();
        let root_cause = error.root_cause();
        assert_eq!(format!("{}", root_cause), "usage: 
            tr [-Ccsu] string1 string2
            tr [-Ccu] -d string1
            tr [-Ccu] -s string1
            tr [-Ccu] -ds string1 string2"
        );
        
    }

    // ************************************************************************
    // compress mode tests
    // ************************************************************************

    #[test]
    fn can_run_in_compress_mode(){
        
        let args = Cli {
            complement1: false,
            complement2: false,
            delete: false,
            squeeze: true,
            unbuffered: false,
            string1: "c".to_string(),
            string2: None
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::Compress));
        
    }

    // ************************************************************************
    // delete and compress mode tests
    // ************************************************************************

    #[test]
    fn can_run_in_delete_and_compress_mode(){
        
        let args = Cli {
            complement1: true,
            complement2: true,
            delete: true,
            squeeze: true,
            unbuffered: true,
            string1: "c".to_string(),
            string2: Some("C".to_string())
        };

        let mode = init(&args).unwrap();

        assert!(matches!(mode, Mode::DeleteCompress));
        
    }
    
}