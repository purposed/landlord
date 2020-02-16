use clap::ArgMatches;

use rood::CausedResult;

use super::{publish, release, validate};

pub fn run_main(matches: ArgMatches) -> CausedResult<()> {
    match matches.subcommand() {
        ("publish", Some(m)) => publish(m),
        ("release", Some(m)) => release(m),
        ("validate", Some(m)) => validate(m),
        _ => Ok(()),
    }
}
