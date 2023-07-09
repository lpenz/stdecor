// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::Result;

use stdecor::decor::Decor;

#[test]
fn test_decor() -> Result<()> {
    let decor = Decor::new("1234", false, None)?;
    assert_eq!(
        decor.decorate("abcd").collect::<Vec<_>>(),
        vec!["1234 abcd\n"]
    );
    Ok(())
}

#[test]
fn test_decor_wrap() -> Result<()> {
    let decor = Decor::new("1234", false, Some(7))?;
    assert_eq!(
        decor.decorate("abcde").collect::<Vec<_>>(),
        vec!["1234 ab\n", "1234 cd\n", "1234 e\n"]
    );
    Ok(())
}
