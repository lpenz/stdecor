// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::Parser;
use std::error::Error;
use std::process;
use terminal_size::{Width, terminal_size};

pub mod decor;
pub mod pipe;
pub mod runner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Add a default prefix to both stdout and stderr
    #[arg(short, long, default_value = "")]
    pub prefix: String,

    /// Add the date and time as a prefix to both stdout and stderr
    #[arg(short, long, default_value_t = false)]
    pub date: bool,

    /// The command to run; use stdin if empty (pipe mode)
    pub command: Vec<String>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();
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
