// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::shells::Bash;
use clap_complete::shells::Fish;
use clap_complete::shells::Zsh;
use color_eyre::{Result, eyre::eyre};
use man::prelude::*;
use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path;

include!("src/cli.rs");

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
        .arg(Arg::new("COMMAND"))
        .arg(Arg::new("[ ARGS ]"))
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
    // Generate shell completions:
    let mut cmd = Cli::command();
    generate_to(Bash, &mut cmd, "stdecor", &outdir)?;
    let path = generate_to(Fish, &mut cmd, "stdecor", &outdir)?;
    let mut fd = OpenOptions::new().append(true).open(path)?;
    writeln!(fd, "complete -c stdecor --wraps command")?;
    writeln!(fd, "complete -c stdecor --no-files")?;
    generate_to(Zsh, &mut cmd, "stdecor", &outdir)?;
    Ok(())
}
