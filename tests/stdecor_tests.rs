// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;

use stdecor::decor::Decor;
use stdecor::*;

#[test]
fn test_decor() -> Result<()> {
    let decor = Decor::new("1234", false);
    assert_eq!(
        decor.decorate("abcd").collect::<Vec<_>>(),
        vec!["1234 abcd\n"]
    );
    Ok(())
}

#[tokio::test]
async fn test_writer_async() -> Result<()> {
    assert_eq!(
        &writer_async::decor_str("1234", false, "abcd").await?,
        "1234 abcd\n"
    );
    Ok(())
}
