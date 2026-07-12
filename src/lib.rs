// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::Parser;
use std::error::Error;
use std::process;
use terminal_size::{Width, terminal_size};

mod cli;
mod decor;
mod pipe;
mod runner;

pub fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = cli::Cli::parse();
    let width = if let Some((Width(w), _)) = terminal_size() {
        Some(w as usize)
    } else {
        None
    };
    if cli.command.is_empty() {
        pipe::pipe(&cli.prefix, cli.date, width)?;
        Ok(())
    } else {
        let command: Vec<&str> = cli.command.iter().map(String::as_ref).collect();
        let exitstatus = runner::run(&cli.prefix, cli.date, width, &command)?;
        process::exit(exitstatus.code().unwrap_or(0));
    }
}

#[cfg(test)]
mod tests {
    use super::decor::{Decor, LineWrapper};
    use color_eyre::Result;

    #[test]
    fn test_decor() -> Result<()> {
        let decor = Decor::new("1234", false, None)?;
        assert_eq!(
            decor.decorate("abcd").collect::<Vec<_>>(),
            vec!["1234 abcd\n"]
        );
        Ok(())
    }

    #[test]
    fn test_decor_wrap() -> Result<()> {
        let decor = Decor::new("1234", false, Some(7))?;
        assert_eq!(
            decor.decorate("abcde").collect::<Vec<_>>(),
            vec!["1234 ab\n", "1234 cd\n", "1234 e\n"]
        );
        Ok(())
    }

    #[test]
    fn test_decor_wrap_multibyte() -> Result<()> {
        // "p " is 2 chars, width 3 → 1 char of content per line
        let decor = Decor::new("p", false, Some(3))?;
        assert_eq!(
            decor.decorate("日本語").collect::<Vec<_>>(),
            vec!["p 日\n", "p 本\n", "p 語\n"]
        );
        Ok(())
    }

    #[test]
    fn test_decor_multibyte_prefix() -> Result<()> {
        // "日" is 3 bytes but 1 char — prefix fits in width 5
        let decor = Decor::new("日", false, Some(5))?;
        assert_eq!(
            decor.decorate("abcde").collect::<Vec<_>>(),
            vec!["日 abc\n", "日 de\n"]
        );
        Ok(())
    }

    #[test]
    fn test_decor_multibyte_prefix_overflow() -> Result<()> {
        // "日本語" = 9 bytes, 3 chars — too wide for terminal of 3
        let result = Decor::new("日本語", false, Some(3));
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_decor_multibyte_no_unnecessary_wrap() -> Result<()> {
        // "日本" = 6 bytes, 2 chars — fits in width 5, no wrapping needed
        let decor = Decor::new("", false, Some(5))?;
        assert_eq!(decor.decorate("日本").collect::<Vec<_>>(), vec!["日本\n"]);
        Ok(())
    }

    #[test]
    fn test_decor_date() -> Result<()> {
        let decor = Decor::new("", true, None)?;
        let lines: Vec<_> = decor.decorate("hello").collect();
        assert_eq!(lines.len(), 1);
        assert!(
            lines[0].starts_with("20"),
            "expected timestamp prefix, got: {}",
            lines[0]
        );
        assert!(lines[0].contains("hello"));
        assert!(lines[0].ends_with('\n'));
        Ok(())
    }

    #[test]
    fn test_decor_date_with_prefix() -> Result<()> {
        let decor = Decor::new("tag", true, None)?;
        let lines: Vec<_> = decor.decorate("msg").collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("20"));
        assert!(lines[0].contains("tag "));
        assert!(lines[0].contains("msg"));
        Ok(())
    }

    #[test]
    fn test_decor_date_with_width() -> Result<()> {
        let decor = Decor::new("", true, Some(50))?;
        let lines: Vec<_> = decor.decorate("short").collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("short"));
        Ok(())
    }

    #[test]
    fn test_decor_date_prefix_overflow() -> Result<()> {
        // date prefix alone is ~26 chars, width of 5 is way too small
        let result = Decor::new("", true, Some(5));
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_decor_date_prefix_with_prefix_overflow() -> Result<()> {
        // date (~26) + "prefix " (~7) = ~33, width 20 too small
        let result = Decor::new("prefix", true, Some(20));
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_decor_empty_line() -> Result<()> {
        let decor = Decor::new("p", false, None)?;
        let lines: Vec<_> = decor.decorate("").collect();
        assert_eq!(lines, vec!["p \n"]);
        Ok(())
    }

    #[test]
    fn test_decor_empty_prefix() -> Result<()> {
        let decor = Decor::new("", false, None)?;
        assert_eq!(decor.decorate("hello").collect::<Vec<_>>(), vec!["hello\n"]);
        Ok(())
    }

    #[test]
    fn test_line_wrapper_no_width() -> Result<()> {
        let wrapper = LineWrapper::new("abc\ndef", None);
        assert_eq!(wrapper.collect::<Vec<_>>(), vec!["abc\ndef"]);
        Ok(())
    }

    #[test]
    fn test_line_wrapper_zero_width() -> Result<()> {
        // width=0: nth(0) returns first char index (0), current is empty → None
        let wrapper = LineWrapper::new("abc", Some(0));
        assert_eq!(wrapper.collect::<Vec<_>>(), Vec::<&str>::new());
        Ok(())
    }

    #[test]
    fn test_line_wrapper_empty_string() -> Result<()> {
        let wrapper = LineWrapper::new("", Some(10));
        assert_eq!(wrapper.collect::<Vec<_>>(), Vec::<&str>::new());
        Ok(())
    }

    #[test]
    fn test_line_wrapper_empty_string_no_width() -> Result<()> {
        let wrapper = LineWrapper::new("", None);
        assert_eq!(wrapper.collect::<Vec<_>>(), vec![""]);
        Ok(())
    }

    #[test]
    fn test_line_wrapper_exact_fit() -> Result<()> {
        let wrapper = LineWrapper::new("abc", Some(3));
        assert_eq!(wrapper.collect::<Vec<_>>(), vec!["abc"]);
        Ok(())
    }

    #[test]
    fn test_line_wrapper_one_over() -> Result<()> {
        let wrapper = LineWrapper::new("abcd", Some(3));
        assert_eq!(wrapper.collect::<Vec<_>>(), vec!["abc", "d"]);
        Ok(())
    }

    #[test]
    fn test_line_wrapper_multibyte_exact_fit() -> Result<()> {
        // "日本" = 2 chars, width=2
        let wrapper = LineWrapper::new("日本", Some(2));
        assert_eq!(wrapper.collect::<Vec<_>>(), vec!["日本"]);
        Ok(())
    }

    #[test]
    fn test_line_wrapper_multibyte_split() -> Result<()> {
        // "日本語" = 3 chars, width=2 → "日本", "語"
        let wrapper = LineWrapper::new("日本語", Some(2));
        assert_eq!(wrapper.collect::<Vec<_>>(), vec!["日本", "語"]);
        Ok(())
    }

    #[test]
    fn test_line_wrapper_single_char_width() -> Result<()> {
        let wrapper = LineWrapper::new("abc", Some(1));
        assert_eq!(wrapper.collect::<Vec<_>>(), vec!["a", "b", "c"]);
        Ok(())
    }
}
