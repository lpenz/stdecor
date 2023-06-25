// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use chrono;
use color_eyre::{eyre::eyre, Result};
use std::convert::TryFrom;
use std::process::ExitStatus;
use std::process::Stdio;
use tokio::io::{self, AsyncWriteExt};
use tokio::process::Command;
use tokio_process_stream as tps;
use tokio_stream::StreamExt;

use crate::cli::Cli;

#[tracing::instrument]
pub fn buildcmd(cli: &Cli) -> Command {
    let mut cmd = Command::new(&cli.command[0]);
    cmd.args(cli.command.iter().skip(1))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}

pub async fn do_write<T>(mut fd: T, prefix: &str, date: bool, line: &str) -> Result<()>
where
    T: AsyncWriteExt + std::marker::Unpin,
{
    let mut string = String::new();
    if date {
        let now = chrono::offset::Local::now();
        string.push_str(&now.format("%Y-%m-%d %H:%M:%S%.6f ").to_string());
    }
    string.push_str(prefix);
    string.push_str(line);
    string.push('\n');
    fd.write_all(string.as_bytes()).await.map_err(|e| eyre!(e))
}

#[tracing::instrument]
pub async fn run(cli: &Cli) -> Result<ExitStatus> {
    let cmd = buildcmd(cli);
    let mut stream = tps::ProcessStream::try_from(cmd)?;
    let prefix_static = if let Some(prefix) = &cli.prefix {
        format!("{} ", prefix)
    } else {
        "".to_string()
    };
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    while let Some(item) = stream.next().await {
        match item {
            tps::Item::Stdout(line) => {
                do_write(&mut stdout, &prefix_static, cli.date, &line).await?;
            }
            tps::Item::Stderr(line) => {
                do_write(&mut stderr, &prefix_static, cli.date, &line).await?;
            }
            tps::Item::Done(s) => {
                return Ok(s?);
            }
        }
    }
    Err(eyre!("stream exhausted without a \"done\" element"))
}
