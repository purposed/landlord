use clap::ArgMatches;

use rood::cli::OutputManager;
use rood::{Cause, CausedResult};

use publib::{BuildMode, MetaBuilder, MetaValidator};

fn get_build_mode(matches: &ArgMatches) -> BuildMode {
    if matches.is_present("release") || !matches.is_present("debug") {
        BuildMode::Release
    } else {
        BuildMode::Debug
    }
}

fn build(matches: &ArgMatches, output: &OutputManager) -> CausedResult<()> {
    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.
    let project = publib::Project::new(project_path)?;

    let builder = MetaBuilder::default();
    builder.build(&project, get_build_mode(matches), &output)?;

    Ok(())
}

fn validate(matches: &ArgMatches, output: &OutputManager) -> CausedResult<()> {
    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.
    let project = publib::Project::new(project_path)?;

    let validator = MetaValidator::default();
    validator.validate(&project, output)
}

pub fn publish(matches: &ArgMatches) -> CausedResult<()> {
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);
    output.step("[Publish]");

    build(matches, &output.push())?;
    validate(matches, &output.push())?;

    output.success("[Publish] - OK");

    Ok(())
}
