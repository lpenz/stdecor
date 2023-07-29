// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{eyre::eyre, Result};
use lineriver::{LineReadFd, LineReader};
use polling::{Event, Poller};
use std::io::{self, Write};
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;

use crate::decor::Decor;

#[tracing::instrument]
pub fn buildcmd(command: &[&str]) -> Command {
    let mut cmd = Command::new(command[0]);
    cmd.args(command.iter().skip(1))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}

fn print_lines(decor: &Decor, key: usize, line: &str) -> Result<()> {
    let mut output: Box<dyn Write> = if key == 0 {
        Box::new(io::stdout().lock())
    } else {
        Box::new(io::stderr().lock())
    };
    for line_out in decor.decorate(line) {
        output.write_all(line_out.as_bytes())?;
    }
    output.flush()?;
    Ok(())
}

#[tracing::instrument]
pub fn run(prefix: &str, date: bool, width: Option<usize>, command: &[&str]) -> Result<ExitStatus> {
    let decor = Decor::new(prefix, date, width)?;
    let mut child = buildcmd(command).spawn()?;
    let child_stdout = LineReader::new(
        child
            .stdout
            .take()
            .ok_or_else(|| eyre!("error taking stdout"))?,
    )?;
    let child_stderr = LineReader::new(
        child
            .stderr
            .take()
            .ok_or_else(|| eyre!("error taking stderr"))?,
    )?;
    /* stdout and stderr have different types.
     * Let's erase their types to handle them with the
     * same code: */
    let mut linereaders: Vec<Box<dyn LineReadFd>> =
        vec![Box::new(child_stdout), Box::new(child_stderr)];
    let poller = Poller::new()?;
    for (key, linereader) in linereaders.iter().enumerate() {
        poller.add(linereader.as_raw_fd(), Event::readable(key))?;
    }
    let mut events = Vec::new();
    loop {
        events.clear();
        poller.wait(&mut events, None)?;
        for ev in &events {
            let linereader = &mut linereaders[ev.key];
            if !linereader.eof() {
                linereader.read_available()?;
                for line in linereader.lines_get() {
                    print_lines(&decor, ev.key, &line)?;
                }
                // Set interest in the next readability event from client.
                poller.modify(linereaders[ev.key].as_raw_fd(), Event::readable(ev.key))?;
            }
        }
        if let Some(result) = child.try_wait()? {
            // Child exited, print all pending output.
            // Specially important if the command doesn't end its output with a newline.
            for mut linereader in linereaders.into_iter() {
                linereader.read_available()?;
                for (key, line) in linereader.lines_get().into_iter().enumerate() {
                    print_lines(&decor, key, &line)?;
                }
            }
            return Ok(result);
        }
    }
}
