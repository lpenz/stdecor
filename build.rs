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
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path;

include!("src/cli.rs");

fn generate_man_page<P: AsRef<path::Path>>(outdir: P) -> Result<()> {
    let cmd = Cli::command();
    let name = cmd
        .get_display_name()
        .unwrap_or_else(|| cmd.get_name())
        .to_owned();
    let outdir = outdir.as_ref();
    let man_path = outdir.join(format!("{}.1", name));
    let manual = clap2man::Manual::from(&cmd);
    let mut manpage: man::Manual = manual.into();
    manpage = manpage
        .example(
            Example::new()
                .text(r#"Run 2 "find" commands for different directories"#)
                .command("stdecor -p [var] find /var & stdecor -p [usr] find /usr & wait"),
        )
        .example(
            Example::new()
                .text("Update 2 docker images, showing the dates")
                .command("stdecor -d -p [bookworm] docker pull debian:bookworm & stdecor -d -p [stretch] docker pull debian:stretch & wait"),
        );
    std::fs::write(man_path, manpage.render())?;
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
