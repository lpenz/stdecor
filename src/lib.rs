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
    use super::decor::Decor;
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
}
