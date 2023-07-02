// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use chrono;
use std::iter;

pub struct Decor {
    prefix: String,
    date: bool,
}

impl Decor {
    pub fn new(prefix: &str, date: bool) -> Self {
        Decor {
            prefix: prefix.to_string(),
            date,
        }
    }

    pub fn decorate(&self, line: &str) -> impl iter::Iterator<Item = String> {
        let mut output = String::new();
        if self.date {
            let now = chrono::offset::Local::now();
            let date = now.format("%Y-%m-%d %H:%M:%S%.6f ").to_string();
            output.push_str(&date);
        }
        output.push_str(&self.prefix);
        output.push(' ');
        output.push_str(line);
        output.push('\n');
        iter::once(output)
    }
}
