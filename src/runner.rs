// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

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
    cmd.args(cli.command.iter().skip(1));
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd
}

pub async fn do_write<T>(mut fd: T, prefix: &str, line: &str) -> Result<()>
where
    T: AsyncWriteExt + std::marker::Unpin,
{
    let string = format!("{}{}\n", prefix, line);
    fd.write_all(string.as_bytes()).await.map_err(|e| eyre!(e))
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
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    while let Some(item) = stream.next().await {
        match item {
            tps::Item::Stdout(line) => {
                do_write(&mut stdout, &prefix, &line).await?;
            }
            tps::Item::Stderr(line) => {
                do_write(&mut stderr, &prefix, &line).await?;
            }
            tps::Item::Done(s) => {
                return Ok(s?);
            }
        }
    }
    Err(eyre!("stream exhausted without a \"done\" element"))
}
