// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use tokio::io::{self, AsyncWriteExt};

use crate::decor::Decor;

pub async fn decor_write<T>(decor: &Decor, line: &str, output: &mut io::BufWriter<T>) -> Result<()>
where
    T: AsyncWriteExt + std::marker::Unpin,
{
    for subline in decor.decorate(line) {
        output.write_all(subline.as_bytes()).await?;
    }
    Ok(())
}

pub async fn decor_str(prefix: &str, date: bool, line: &str) -> Result<String> {
    let decor = Decor::new(prefix, date);
    let mut output = Vec::<u8>::new();
    let mut o = io::BufWriter::new(&mut output);
    for line in decor.decorate(line) {
        o.write_all(line.as_bytes()).await?;
    }
    o.flush().await?;
    Ok(std::str::from_utf8(&output)?.to_owned())
}
