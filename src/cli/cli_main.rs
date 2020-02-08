use clap::ArgMatches;

use rood::CausedResult;

use super::build;

pub fn run_main(matches: ArgMatches) -> CausedResult<()> {
    match matches.subcommand() {
        ("build", Some(m)) => build(m),
        _ => Ok(()),
    }
}
