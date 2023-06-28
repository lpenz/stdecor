// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use chrono;
use color_eyre::{eyre::eyre, Result};
use std::convert::TryFrom;
use std::process::ExitStatus;
use std::process::Stdio;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio_process_stream as tps;
use tokio_stream::wrappers::LinesStream;
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

pub fn decorate(prefix: &str, date: bool, line: &str) -> String {
    let mut string = String::new();
    if date {
        let now = chrono::offset::Local::now();
        string.push_str(&now.format("%Y-%m-%d %H:%M:%S%.6f ").to_string());
    }
    string.push_str(prefix);
    string.push(' ');
    string.push_str(line);
    string.push('\n');
    string
}

pub async fn do_write<T>(mut fd: T, prefix: &str, date: bool, line: &str) -> Result<()>
where
    T: AsyncWriteExt + std::marker::Unpin,
{
    let string = decorate(prefix, date, line);
    fd.write_all(string.as_bytes()).await.map_err(|e| eyre!(e))
}

#[tracing::instrument]
pub async fn run(cli: &Cli) -> Result<ExitStatus> {
    let cmd = buildcmd(cli);
    let mut stream = tps::ProcessStream::try_from(cmd)?;
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    while let Some(item) = stream.next().await {
        match item {
            tps::Item::Stdout(line) => {
                do_write(&mut stdout, &cli.prefix, cli.date, &line).await?;
            }
            tps::Item::Stderr(line) => {
                do_write(&mut stderr, &cli.prefix, cli.date, &line).await?;
            }
            tps::Item::Done(s) => {
                return Ok(s?);
            }
        }
    }
    Err(eyre!("stream exhausted without a \"done\" element"))
}

#[tracing::instrument]
pub async fn pipe(cli: &Cli) -> Result<()> {
    let mut stdin_lines = LinesStream::new(BufReader::new(io::stdin()).lines());
    let mut stdout = io::stdout();
    while let Some(line) = stdin_lines.next().await {
        do_write(&mut stdout, &cli.prefix, cli.date, &line?).await?;
    }
    Ok(())
}
