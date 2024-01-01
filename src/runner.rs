// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{eyre::eyre, Result};
use lineriver::{LineReadRawAndFd, LineReader};
use polling::{Event, Events, Poller};
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

fn print_line(decor: &Decor, key: usize, line: &str) -> Result<()> {
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

fn read_print_lines(
    decor: &Decor,
    key: usize,
    linereader: &mut Box<dyn LineReadRawAndFd>,
) -> Result<()> {
    linereader.read_available()?;
    for line in linereader.lines_get() {
        print_line(&decor, key, &line)?;
    }
    Ok(())
}

fn read_print_flush(decor: &Decor, linereaders: Vec<Box<dyn LineReadRawAndFd>>) -> Result<()> {
    for (key, mut linereader) in linereaders.into_iter().enumerate() {
        read_print_lines(decor, key, &mut linereader)?;
    }
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
    let mut linereaders: Vec<Box<dyn LineReadRawAndFd>> =
        vec![Box::new(child_stdout), Box::new(child_stderr)];
    let poller = Poller::new()?;
    for (key, linereader) in linereaders.iter().enumerate() {
        unsafe {
            poller.add(linereader.as_raw_fd(), Event::readable(key))?;
        };
    }
    let mut events = Events::new();
    loop {
        events.clear();
        poller.wait(&mut events, None)?;
        for ev in events.iter() {
            let linereader = &mut linereaders[ev.key];
            if !linereader.eof() {
                read_print_lines(&decor, ev.key, linereader)?;
                // Set interest in the next readability event from client.
                poller.modify(linereaders[ev.key].as_fd(), Event::readable(ev.key))?;
            }
        }
        if let Some(result) = child.try_wait()? {
            // Child exited, print all pending output.
            // Specially important if the command doesn't end its
            // output with a newline.
            read_print_flush(&decor, linereaders)?;
            return Ok(result);
        } else if linereaders.iter().all(|lr| lr.eof()) {
            // Both stdout and stderr at eof, nothing to do except
            // wait for child process.
            let result = child.wait()?;
            read_print_flush(&decor, linereaders)?;
            return Ok(result);
        }
    }
}
