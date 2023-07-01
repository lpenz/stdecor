// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;

use stdecor::*;

#[tokio::test]
async fn test_decor() -> Result<()> {
    assert_eq!(
        &decor_async::decor_str("1234", false, "abcd").await?,
        "1234 abcd\n"
    );
    Ok(())
}
