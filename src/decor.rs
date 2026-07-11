// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{Result, eyre::eyre};
use std::iter;

#[derive(Debug)]
pub struct Decor {
    prefix: String,
    date: bool,
    width: Option<usize>,
}

#[inline]
fn gen_fullprefix(prefix: &str, date: bool) -> String {
    let mut fullprefix = String::new();
    if date {
        let now = chrono::offset::Local::now();
        let date = now.format("%Y-%m-%dT%H:%M:%S%.3f%:z ").to_string();
        fullprefix.push_str(&date);
    }
    if !prefix.is_empty() {
        fullprefix.push_str(prefix);
        fullprefix.push(' ');
    }
    fullprefix
}

impl Decor {
    #[tracing::instrument(ret, err)]
    pub fn new(prefix: &str, date: bool, width: Option<usize>) -> Result<Self> {
        if let Some(w) = width {
            let prefixlen = gen_fullprefix(prefix, date).chars().count();
            if prefixlen >= w {
                return Err(eyre!(
                    "prefix with {} chars is too big for line width {}",
                    prefixlen,
                    w
                ));
            }
        }
        Ok(Decor {
            prefix: prefix.to_string(),
            date,
            width,
        })
    }

    #[tracing::instrument(level = "trace", ret)]
    pub fn decorate<'a>(&self, line: &'a str) -> impl iter::Iterator<Item = String> + 'a {
        let fullprefix = gen_fullprefix(&self.prefix, self.date);
        LineWrapper::new(line, self.width.map(|w| w - fullprefix.chars().count())).map(move |l| {
            if l.ends_with('\n') {
                format!("{}{}", fullprefix, l)
            } else {
                format!("{}{}\n", fullprefix, l)
            }
        })
    }
}

#[derive(Debug)]
pub struct LineWrapper<'a> {
    rest: Option<&'a str>,
    width: Option<usize>,
}

impl<'a> LineWrapper<'a> {
    #[tracing::instrument(level = "trace", ret)]
    pub fn new(original: &'a str, width: Option<usize>) -> Self {
        Self {
            rest: Some(original),
            width,
        }
    }
}

impl<'a> Iterator for LineWrapper<'a> {
    type Item = &'a str;

    #[tracing::instrument(level = "trace", ret)]
    fn next(&mut self) -> Option<Self::Item> {
        let rest = self.rest.as_mut()?;
        if let Some(width) = self.width {
            if rest.len() >= width {
                let end = rest.floor_char_boundary(width);
                let current = &rest[0..end];
                *rest = &rest[end..];
                if rest.is_empty() {
                    self.rest = None;
                }
                Some(current)
            } else {
                self.rest.take()
            }
        } else {
            self.rest.take()
        }
    }
}
