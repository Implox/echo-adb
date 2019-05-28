extern crate regex;

use regex::*;
use std::process::{Command, Output};

type EscapedString = String;

pub fn escape_text(input_str: String) -> EscapedString {
    let space_re = Regex::new(r" ").unwrap();
    let symb_re = Regex::new("([\\()<>|;&*~\"\'])").unwrap();
    let space_str = space_re.replace_all(&input_str, "%s");
    return symb_re.replace_all(&space_str, r"\$1").into_owned();
}

pub fn echo_adb(str: EscapedString) -> Output {
    return Command::new("adb")
        .args(&["shell", "input", "text", &str])
        .output()
        .expect("Failed to echo to device.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_paren() {
        let input = "(".to_string();
        assert_eq!(escape_text(input), r"\(");
    }

    #[test]
    fn test_close_paren() {
        let input = ")".to_string();
        assert_eq!(escape_text(input), r"\)");
    }

    #[test]
    fn test_open_chevron() {
        let input = "<".to_string();
        assert_eq!(escape_text(input), r"\<");
    }

    #[test]
    fn test_close_chevron() {
        let input = ">".to_string();
        assert_eq!(escape_text(input), r"\>");
    }

    #[test]
    fn test_pipe() {
        let input = "|".to_string();
        assert_eq!(escape_text(input), r"\|");
    }

    #[test]
    fn test_semicolon() {
        let input = ";".to_string();
        assert_eq!(escape_text(input), r"\;");
    }

    #[test]
    fn test_ampersand() {
        let input = "&".to_string();
        assert_eq!(escape_text(input), r"\&");
    }

    #[test]
    fn test_star() {
        let input = "*".to_string();
        assert_eq!(escape_text(input), r"\*");
    }

    #[test]
    fn test_backslash() {
        let input = r"\".to_string();
        assert_eq!(escape_text(input), "\\");
    }

    #[test]
    fn test_tilde() {
        let input = "~".to_string();
        assert_eq!(escape_text(input), r"\~");
    }

    #[test]
    fn test_quote() {
        let input = "\"".to_string();
        assert_eq!(escape_text(input), "\\\"");
    }

    #[test]
    fn test_apostrophe() {
        let input = "'".to_string();
        assert_eq!(escape_text(input), "\\'");
    }

    #[test]
    fn test_space() {
        let input = " ".to_string();
        assert_eq!(escape_text(input), "%s");
    }
}
