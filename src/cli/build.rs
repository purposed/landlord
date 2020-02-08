use clap::ArgMatches;

use rood::cli::OutputManager;
use rood::{Cause, CausedResult, Error};

use publib;
use publib::BuildMode;

pub fn build(matches: &ArgMatches) -> CausedResult<()> {
    let verbose = matches.is_present("verbose");
    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.

    let output = OutputManager::new(verbose);
    output.step("BING YEET", 0);

    let project = publib::Project::new(project_path)?;

    let executor = publib::default_executor();
    executor.build(&project, BuildMode::Release)?;

    Ok(())
}
