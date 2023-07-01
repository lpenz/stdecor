// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use chrono;
use color_eyre::{eyre::eyre, Result};
use std::convert::TryFrom;
use std::process::ExitStatus;
use std::process::Stdio;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
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

pub async fn decor_write<T>(
    prefix: &str,
    date: bool,
    line: &str,
    output: &mut io::BufWriter<T>,
) -> Result<()>
where
    T: AsyncWriteExt + std::marker::Unpin,
{
    if date {
        let now = chrono::offset::Local::now();
        let date = now.format("%Y-%m-%d %H:%M:%S%.6f ").to_string();
        output.write_all(date.as_bytes()).await?;
    }
    output.write_all(prefix.as_bytes()).await?;
    output.write_all(&[b' ']).await?;
    output.write_all(line.as_bytes()).await?;
    output.write_all(&[b'\n']).await?;
    Ok(())
}

pub async fn decor_str(prefix: &str, date: bool, line: &str) -> Result<String> {
    let mut output = Vec::<u8>::new();
    let mut o = io::BufWriter::new(&mut output);
    decor_write(prefix, date, line, &mut o).await?;
    o.flush().await?;
    Ok(std::str::from_utf8(&output)?.to_owned())
}

#[tracing::instrument]
pub async fn run(cli: &Cli) -> Result<ExitStatus> {
    let cmd = buildcmd(cli);
    let mut stream = tps::ProcessStream::try_from(cmd)?;
    let mut stdout = BufWriter::new(io::stdout());
    let mut stderr = BufWriter::new(io::stderr());
    while let Some(item) = stream.next().await {
        match item {
            tps::Item::Stdout(line) => {
                decor_write(&cli.prefix, cli.date, &line, &mut stdout).await?;
            }
            tps::Item::Stderr(line) => {
                decor_write(&cli.prefix, cli.date, &line, &mut stderr).await?;
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
    let mut stdout = BufWriter::new(io::stdout());
    while let Some(line) = stdin_lines.next().await {
        decor_write(&cli.prefix, cli.date, &line?, &mut stdout).await?;
    }
    Ok(())
}
