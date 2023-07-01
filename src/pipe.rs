// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;
use tokio::io::{self, AsyncBufReadExt, BufReader, BufWriter};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

use crate::cli::Cli;
use crate::decor_async;

#[tracing::instrument]
pub async fn pipe(cli: &Cli) -> Result<()> {
    let mut stdin_lines = LinesStream::new(BufReader::new(io::stdin()).lines());
    let mut stdout = BufWriter::new(io::stdout());
    while let Some(line) = stdin_lines.next().await {
        decor_async::decor_write(&cli.prefix, cli.date, &line?, &mut stdout).await?;
    }
    Ok(())
}
