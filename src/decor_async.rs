// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use chrono;
use color_eyre::Result;
use tokio::io::{self, AsyncWriteExt};

pub async fn decor_write<T>(
    prefix: &str,
    date: bool,
    line: &str,
    output: &mut io::BufWriter<T>,
) -> Result<()>
where
    T: AsyncWriteExt + std::marker::Unpin,
{
    if date {
        let now = chrono::offset::Local::now();
        let date = now.format("%Y-%m-%d %H:%M:%S%.6f ").to_string();
        output.write_all(date.as_bytes()).await?;
    }
    output.write_all(prefix.as_bytes()).await?;
    output.write_all(&[b' ']).await?;
    output.write_all(line.as_bytes()).await?;
    output.write_all(&[b'\n']).await?;
    Ok(())
}

pub async fn decor_str(prefix: &str, date: bool, line: &str) -> Result<String> {
    let mut output = Vec::<u8>::new();
    let mut o = io::BufWriter::new(&mut output);
    decor_write(prefix, date, line, &mut o).await?;
    o.flush().await?;
    Ok(std::str::from_utf8(&output)?.to_owned())
}
