use clap::ArgMatches;

use anyhow::Result;

use super::{publish, release, validate};

pub fn run_main(matches: ArgMatches) -> Result<()> {
    if let Some(subcmd) = matches.subcommand() {
        match subcmd {
            ("publish", m) => publish(m),
            ("release", m) => release(m),
            ("validate", m) => validate(m),
            _ => Ok(()),
        }
    } else {
        Ok(())
    }
}
