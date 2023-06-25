// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{eyre::eyre, Result};
use std::convert::TryFrom;
use std::process::ExitStatus;
use std::process::Stdio;
use tokio::process::Command;
use tokio_process_stream as tps;
use tokio_stream::StreamExt;

use crate::cli::Cli;

#[tracing::instrument]
pub fn buildcmd(cli: &Cli) -> Command {
    let mut cmd = Command::new(&cli.command[0]);
    cmd.args(cli.command.iter().skip(1));
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd
}

#[tracing::instrument]
pub async fn run(cli: &Cli) -> Result<ExitStatus> {
    let cmd = buildcmd(cli);
    let mut stream = tps::ProcessStream::try_from(cmd)?;
    let prefix = if let Some(prefix) = &cli.prefix {
        format!("{} ", prefix)
    } else {
        "".to_string()
    };
    while let Some(item) = stream.next().await {
        match item {
            tps::Item::Stdout(l) => {
                println!("{}{}", prefix, l);
            }
            tps::Item::Stderr(l) => {
                eprintln!("{}{}", prefix, l);
            }
            tps::Item::Done(s) => {
                return Ok(s?);
            }
        }
    }
    Err(eyre!("stream exhausted without a \"done\" element"))
}
