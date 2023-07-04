// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use tokio::io::AsyncWriteExt;
use tokio::io::{self, AsyncBufReadExt, BufReader, BufWriter};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

use crate::decor::Decor;
use crate::writer_async;

#[tracing::instrument]
pub async fn pipe(prefix: &str, date: bool, width: Option<usize>) -> Result<()> {
    let decor = Decor::new(prefix, date, width)?;
    let mut stdin_lines = LinesStream::new(BufReader::new(io::stdin()).lines());
    let mut stdout = BufWriter::new(io::stdout());
    while let Some(line) = stdin_lines.next().await {
        writer_async::decor_write(&decor, &line?, &mut stdout).await?;
    }
    stdout.flush().await?;
    Ok(())
}
