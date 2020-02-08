use clap::ArgMatches;

use rood::cli::OutputManager;
use rood::{Cause, CausedResult, Error};

use publib;
use publib::BuildMode;

fn get_build_mode(matches: &ArgMatches) -> BuildMode {
    if matches.is_present("debug") || !matches.is_present("release") {
        BuildMode::Debug
    } else {
        BuildMode::Release
    }
}

pub fn build(matches: &ArgMatches) -> CausedResult<()> {
    let verbose = matches.is_present("verbose");
    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.

    let output = OutputManager::new(verbose);

    let project = publib::Project::new(project_path)?;

    publib::default_executor().build(&project, get_build_mode(matches), &output)?;

    Ok(())
}
