// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::{eyre::eyre, Result};
use man::prelude::*;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path;

fn generate_man_page<P: AsRef<path::Path>>(outdir: P) -> Result<()> {
    let outdir = outdir.as_ref();
    let man_path = outdir.join("stdecor.1");
    let manpage = Manual::new("stdecor")
        .about("Run a command with a decorated stdout/stderr")
        .author(Author::new("Leandro Lisboa Penz").email("lpenz@lpenz.org"))
        .flag(
            Flag::new()
                .short("-p")
                .long("--prefix")
                .help("Add a default prefix to both stdout and stderr"),
        )
        .flag(
            Flag::new()
                .short("-d")
                .long("--date")
                .help("Add the date and time as a prefix to both stdout and stderr"),
        )
        .flag(
            Flag::new()
                .short("-h")
                .long("--help")
                .help("Prints help information"),
        )
        .flag(
            Flag::new()
                .short("-V")
                .long("--version")
                .help("Prints version information"),
        )
        .arg(Arg::new("[ COMMAND ARGS ]"))
        .description(r#"stdecor is a stream decorator that can add a prefix to each line, the date, etc. It can be used via a pipe or it can the command to be decorater. In the latter case it can decorate stdout and stderr in different ways.

stdecor is specially useful when running multiple jobs in the same shell.
"#)
        .example(
            Example::new()
                .text(r#"Run 2 "find" commands for different directories"#)
                .command("stdecor -p [var] find /var & stdecor -p [usr] find /usr & wait"),
        )
        .example(
            Example::new()
                .text("Update 2 docker images, showing the dates")
                .command("stdecor -d -p [bookworm] docker pull debian:bookworm & stdecor -d -p [stretch] docker pull debian:stretch & wait"),
        )
        .render();
    File::create(man_path)?.write_all(manpage.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    let mut outdir =
        path::PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| eyre!("error getting OUT_DIR"))?);
    fs::create_dir_all(&outdir)?;
    generate_man_page(&outdir)?;
    // build/stdecor-*/out
    outdir.pop();
    // build/stdecor-*
    outdir.pop();
    // build
    outdir.pop();
    // .
    // (either target/release or target/build)
    generate_man_page(&outdir)?;
    Ok(())
}
