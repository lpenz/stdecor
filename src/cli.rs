// Copyright (C) 2026 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::Parser;

/// Run a command with a decorated stdout/stderr
///
/// stdecor is a stream decorator that can add a prefix to each line,
/// the date, etc. It can be used via a pipe or it can the command to
/// be decorater. In the latter case it can decorate stdout and stderr
/// in different ways.
///
/// stdecor is specially useful when running multiple jobs in the same
/// shell.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Add a default prefix to both stdout and stderr
    #[arg(short, long, default_value = "")]
    pub prefix: String,

    /// Add the date and time as a prefix to both stdout and stderr
    #[arg(short, long, default_value_t = false)]
    pub date: bool,

    /// The command to run; use stdin if empty (pipe mode)
    pub command: Vec<String>,
}
