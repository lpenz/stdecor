// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use stdecor::*;

#[test]
fn test_decorate() {
    assert_eq!(&runner::decorate("1234", false, "abcd"), "1234 abcd\n");
}
