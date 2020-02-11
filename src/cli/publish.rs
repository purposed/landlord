use clap::ArgMatches;

use rood::cli::OutputManager;
use rood::{Cause, CausedResult, Error};

use publib::{BuildMode, MetaBuilder, MetaBumper, MetaValidator, Project};

fn get_build_mode(matches: &ArgMatches) -> BuildMode {
    if matches.is_present("release") || !matches.is_present("debug") {
        BuildMode::Release
    } else {
        BuildMode::Debug
    }
}

fn ensure_ready(project: &Project, output: &OutputManager) -> CausedResult<()> {
    // Step 1 - Ensure git repository in project.
    output.step("[Git State]");

    let pushed = output.push();

    if project.repository.has_uncommitted_changes()? {
        return Err(Error::new(
            Cause::InvalidState,
            "Repository has uncommitted changes.",
        ));
    }

    pushed.step("Has No Uncommitted Changes");

    if &project.repository.current_branch()? != "master" {
        return Err(Error::new(
            Cause::InvalidState,
            "Repository is not on master",
        ));
    }

    pushed.step("Is on master branch");

    output.success("[Git State] - OK");
    Ok(())
}

fn build(matches: &ArgMatches, project: &Project, output: &OutputManager) -> CausedResult<()> {
    let builder = MetaBuilder::default();
    builder.build(&project, get_build_mode(matches), &output)?;

    Ok(())
}

fn validate(project: &Project, output: &OutputManager) -> CausedResult<()> {
    let validator = MetaValidator::default();
    validator.validate(&project, output)
}

fn bump_version(
    project: &mut Project,
    level: &str,
    dry: bool,
    output: &OutputManager,
) -> CausedResult<()> {
    let bumper = MetaBumper::default();
    bumper.bump_version(project, level, dry, output)
}

pub fn publish(matches: &ArgMatches) -> CausedResult<()> {
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);

    let unsafe_git = matches.is_present("unsafe");
    let dry = matches.is_present("dry");

    let level = matches.value_of("level").unwrap(); // Mandatory argument.

    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.
    let mut project = publib::Project::new(project_path)?;

    output.step("[Publish]");

    if !unsafe_git {
        ensure_ready(&project, &output.push())?;
    }

    bump_version(&mut project, level, dry, &output.push())?;

    build(matches, &project, &output.push())?;
    validate(&project, &output.push())?;

    output.success("[Publish] - OK");

    Ok(())
}
