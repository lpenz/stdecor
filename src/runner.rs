// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{eyre::eyre, Result};
use std::convert::TryFrom;
use std::process::ExitStatus;
use std::process::Stdio;
use tokio::io::{self, BufWriter};
use tokio::process::Command;
use tokio_process_stream as tps;
use tokio_stream::StreamExt;

use crate::decor_async;

#[tracing::instrument]
pub fn buildcmd(command: &[&str]) -> Command {
    let mut cmd = Command::new(command[0]);
    cmd.args(command.iter().skip(1))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}

#[tracing::instrument]
pub async fn run(prefix: &str, date: bool, command: &[&str]) -> Result<ExitStatus> {
    let cmd = buildcmd(command);
    let mut stream = tps::ProcessStream::try_from(cmd)?;
    let mut stdout = BufWriter::new(io::stdout());
    let mut stderr = BufWriter::new(io::stderr());
    while let Some(item) = stream.next().await {
        match item {
            tps::Item::Stdout(line) => {
                decor_async::decor_write(prefix, date, &line, &mut stdout).await?;
            }
            tps::Item::Stderr(line) => {
                decor_async::decor_write(prefix, date, &line, &mut stderr).await?;
            }
            tps::Item::Done(s) => {
                return Ok(s?);
            }
        }
    }
    Err(eyre!("stream exhausted without a \"done\" element"))
}
