use clap::ArgMatches;

use rood::cli::OutputManager;
use rood::CausedResult;

use super::publish::{build, validate as val};

pub fn validate(matches: &ArgMatches) -> CausedResult<()> {
    /*
    TODO: This isn't clean & needs a refactor.
    Instead of the Validate CLI command duplicating the build + validate flow of
    the publish command, the publish command should re-use the build & validate flows used here.
    */
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);

    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.
    let project = publib::Project::new(project_path)?;

    output.step("[Validation]");

    build(matches, &project, &output.push())?;
    val(&project, &output.push())?;

    output.success("[Validation] - OK");

    Ok(())
}
