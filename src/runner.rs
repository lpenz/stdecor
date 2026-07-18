// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{Result, eyre::eyre};
use lineriver::{LineReadRawAndFd, LineReader};
use polling::{Event, Events, Poller};
use std::io::{self, Write};
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use tracing::info;

use crate::decor::Decor;

#[tracing::instrument(ret, err)]
pub fn buildcmd(command: &[&str]) -> Result<Command> {
    let mut cmd = Command::new(
        command
            .first()
            .ok_or_else(|| eyre!("no command specified"))?,
    );
    cmd.args(command.iter().skip(1))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    Ok(cmd)
}

#[tracing::instrument(level = "trace", skip(decor, output), err)]
fn print_line(decor: &Decor, output: &mut dyn Write, line: &str) -> Result<()> {
    for line_out in decor.decorate(line) {
        output.write_all(line_out.as_bytes())?;
    }
    output.flush()?;
    Ok(())
}

#[tracing::instrument(level = "trace", skip(decor, output, linereader), err)]
fn read_print_lines(
    decor: &Decor,
    output: &mut dyn Write,
    linereader: &mut Box<dyn LineReadRawAndFd>,
) -> Result<()> {
    linereader.read_available()?;
    for line in linereader.lines_get() {
        print_line(decor, output, &line)?;
    }
    Ok(())
}

#[tracing::instrument(skip_all, err)]
fn read_print_flush(
    decor: &Decor,
    linereaders: Vec<Box<dyn LineReadRawAndFd>>,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> Result<()> {
    for (key, mut linereader) in linereaders.into_iter().enumerate() {
        let output: &mut dyn Write = if key == 0 { stdout } else { stderr };
        read_print_lines(decor, output, &mut linereader)?;
    }
    Ok(())
}

#[tracing::instrument(skip_all, ret, err)]
pub fn run(prefix: &str, date: bool, width: Option<usize>, command: &[&str]) -> Result<ExitStatus> {
    info!(
        prefix = prefix,
        date = date,
        width = width,
        command = ?command
    );
    let decor = Decor::new(prefix, date, width)?;
    let mut child = buildcmd(command)?.spawn()?;
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
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    let poller = Poller::new()?;
    for (key, linereader) in linereaders.iter().enumerate() {
        // SAFETY: The raw fd is valid for the lifetime of linereader,
        // and linereaders outlives the poller.
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
                let output: &mut dyn Write = if ev.key == 0 {
                    &mut stdout
                } else {
                    &mut stderr
                };
                read_print_lines(&decor, output, linereader)?;
                // Set interest in the next readability event from client.
                // Ignore unexpected errors — the read fd should remain
                // valid but this is defensive against OS-level issues.
                let _ = poller.modify(linereaders[ev.key].as_fd(), Event::readable(ev.key));
            } else {
                info!(stream = ev.key, "eof");
            }
        }
        if let Some(result) = child.try_wait()? {
            // Child exited, print all pending output.
            // Specially important if the command doesn't end its
            // output with a newline.
            info!(result = ?result, "from child.try_wait");
            read_print_flush(&decor, linereaders, &mut stdout, &mut stderr)?;
            return Ok(result);
        } else if linereaders.iter().all(|lr| lr.eof()) {
            // Both stdout and stderr at eof, nothing to do except
            // wait for child process.
            info!("both streams hit eof, waiting for child");
            let result = child.wait()?;
            info!(result = ?result, "from child.wait");
            read_print_flush(&decor, linereaders, &mut stdout, &mut stderr)?;
            return Ok(result);
        }
    }
}
