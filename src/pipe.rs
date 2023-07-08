// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use std::io::{self, BufRead, Write};

use crate::decor::Decor;

#[tracing::instrument]
pub fn pipe(prefix: &str, date: bool, width: Option<usize>) -> Result<()> {
    let decor = Decor::new(prefix, date, width)?;
    let stdin = io::stdin().lock();
    for line_in in stdin.lines() {
        let mut stdout = io::stdout().lock();
        let line_in = line_in?;
        for line_out in decor.decorate(&line_in) {
            stdout.write_all(line_out.as_bytes())?;
        }
        stdout.flush()?;
    }
    Ok(())
}
