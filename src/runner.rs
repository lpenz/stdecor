// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{eyre::eyre, Result};
use polling::{Event, Poller};
use std::io::{self, BufRead, BufReader, Write};
use std::os::fd::AsRawFd;
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

trait ReadFd: std::io::Read + AsRawFd {}

impl ReadFd for std::process::ChildStdout {}
impl ReadFd for std::process::ChildStderr {}

fn into_reader(r: impl ReadFd + 'static) -> BufReader<Box<dyn ReadFd>> {
    BufReader::new(Box::new(r))
}

fn print_lines(decor: &Decor, buf: &[u8]) -> Result<()> {
    let line_in = std::str::from_utf8(buf)?;
    let mut stdout = io::stdout().lock();
    for line_out in decor.decorate(line_in) {
        stdout.write_all(line_out.as_bytes())?;
    }
    stdout.flush()?;
    Ok(())
}

#[tracing::instrument]
pub fn run(prefix: &str, date: bool, width: Option<usize>, command: &[&str]) -> Result<ExitStatus> {
    let decor = Decor::new(prefix, date, width)?;
    let mut child = buildcmd(command).spawn()?;
    let child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| eyre!("error taking stdout"))?;
    let child_stderr = child
        .stderr
        .take()
        .ok_or_else(|| eyre!("error taking stderr"))?;
    /* stdout and stderr have different types.
     * Let's erase their types to handle them with the
     * same code: */
    let mut bufreaders = [
        Some(into_reader(child_stdout)),
        Some(into_reader(child_stderr)),
    ];
    let poller = Poller::new()?;
    for (key, bufreader) in bufreaders.iter().enumerate() {
        poller.add(
            bufreader.as_ref().unwrap().get_ref().as_raw_fd(),
            Event::readable(key),
        )?;
    }
    let mut events = Vec::new();
    loop {
        events.clear();
        poller.wait(&mut events, None)?;
        for ev in &events {
            if let Some(mut bufreader) = bufreaders[ev.key].take() {
                // ^ Remove the bufreader from the list to process it,
                // add it back if ! eof.
                let eof = bufreader.fill_buf()?.is_empty();
                if !eof {
                    loop {
                        let buf = bufreader.buffer();
                        match memchr::memchr(b'\n', buf) {
                            Some(i) => {
                                print_lines(&decor, &buf[0..i])?;
                                bufreader.consume(i + 1);
                            }
                            None => {
                                // Buffer has no newline, break loop to read more.
                                break;
                            }
                        }
                    }
                    poller.modify(bufreader.get_ref().as_raw_fd(), Event::readable(ev.key))?;
                    bufreaders[ev.key] = Some(bufreader);
                }
            }
        }
        if let Some(result) = child.try_wait()? {
            // Child exited, print all pending output.
            // Specially important if the command doesn't end its output with a newline.
            for bufreader in bufreaders.into_iter().flatten() {
                for line in bufreader.lines() {
                    let line = line?;
                    print_lines(&decor, line.as_bytes())?;
                }
            }
            return Ok(result);
        }
    }
}
