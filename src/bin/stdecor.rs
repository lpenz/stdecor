// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::Parser;
use std::error::Error;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = stdecor::cli::Cli::parse();
    if args.command.is_empty() {
        stdecor::runner::pipe(&args).await?;
        Ok(())
    } else {
        let exitstatus = stdecor::runner::run(&args).await?;
        process::exit(exitstatus.code().unwrap_or(0));
    }
}
