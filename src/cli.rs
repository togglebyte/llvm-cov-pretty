use std::{
    fs::OpenOptions,
    io::{self, BufWriter, Write},
};

use anyhow::{ensure, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::{CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::Shell;

use crate::styles::Theme;

#[derive(Parser)]
#[command(about, author, version)]
pub struct Cli {
    /// Disable any code highlighting.
    #[arg(long)]
    pub no_highlight: bool,
    /// Disable annotations for missing instantiations.
    #[arg(long)]
    pub no_instantiations: bool,
    /// The highlighting theme to use, if not disabled.
    #[arg(long, default_value_t = Theme::OneHalf)]
    pub theme: Theme,
    /// Input coverage file encoded as JSON, or STDIN if omitted.
    pub input: Option<Utf8PathBuf>,
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate auto-completions scripts for various shells.
    Completions {
        /// Shell to generate an auto-completion script for.
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Generate man pages into the given directory.
    Manpages {
        /// Target directory, that must already exist and be empty.
        #[arg(value_hint = ValueHint::DirPath)]
        dir: Utf8PathBuf,
    },
}

pub fn completions(shell: Shell) {
    clap_complete::generate(
        shell,
        &mut Cli::command(),
        env!("CARGO_PKG_NAME"),
        &mut io::stdout().lock(),
    );
}

pub fn manpages(dir: &Utf8Path) -> Result<()> {
    fn print(dir: &Utf8Path, app: &clap::Command) -> Result<()> {
        let name = app.get_display_name().unwrap_or_else(|| app.get_name());
        let out = dir.join(format!("{name}.1"));
        let mut out = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&out)
                .with_context(|| format!("the file `{out}` already exists"))?,
        );

        clap_mangen::Man::new(app.clone()).render(&mut out)?;
        out.flush()?;

        for sub in app.get_subcommands() {
            print(dir, sub)?;
        }

        Ok(())
    }

    ensure!(dir.try_exists()?, "target directory doesn't exist");

    let mut app = Cli::command();
    app.build();

    print(dir, &app)
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }
}
