// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Add a default prefix to both stdout and stderr
    #[arg(short, long)]
    pub prefix: Option<String>,

    /// The command to run
    #[clap(required = true)]
    pub command: Vec<String>,
}
