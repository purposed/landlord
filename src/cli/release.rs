use std::fs;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

use publib::{BuildConfig, BuildMode, MetaBuilder, Project};

use rood::cli::OutputManager;
use rood::CausedResult;

fn get_build_mode(matches: &ArgMatches) -> BuildMode {
    if matches.is_present("release") || !matches.is_present("debug") {
        BuildMode::Release
    } else {
        BuildMode::Debug
    }
}

fn create_artifact_dir<T>(path: T, output: &OutputManager) -> CausedResult<()>
where
    T: AsRef<Path>,
{
    if path.as_ref().exists() {
        fs::remove_dir_all(path.as_ref())?;
        output.step("Removed existing artifact directory")
    }

    fs::create_dir_all(path.as_ref())?;
    output.step("Created artifact directory");
    Ok(())
}

fn format_artifact_path(template: &str, config: &BuildConfig, build_dir: &str) -> String {
    template
        .replace("$(BUILD)", build_dir)
        .replace("$(PLATFORM)", &config.platform.value())
        .replace("$(ARCHITECTURE)", &config.architecture.value())
        .to_string()
}

fn extract_artifacts(
    build_path: &PathBuf,
    config: &BuildConfig,
    project: &Project,
    output: &OutputManager,
) -> CausedResult<()> {
    output.step(&format!(
        "[Bundle/{}-{}]",
        config.platform, config.architecture
    ));

    let pushed = output.push();

    for artifact in project.lease.artifacts.iter() {
        let artifact_path = format_artifact_path(
            &artifact.path_template,
            config,
            build_path.to_str().unwrap(),
        );
        pushed.step(&format!("Processed artifact {}", artifact_path));
    }

    output.step("[Bundle] - OK");
    Ok(())
}

pub fn release(matches: &ArgMatches) -> CausedResult<()> {
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);

    let mode = get_build_mode(matches);

    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.
    let mut project = publib::Project::new(project_path)?;

    output.step("[Release]");

    // Phase 1 - Ensure artifact dir exists.
    create_artifact_dir(&project.lease.artifact_directory, &output.push())?;

    // Phase 2 - Perform all builds.
    let metabuilder = MetaBuilder::default();
    let build_map = metabuilder.build(&project, mode, &output.push())?;

    for (dir_path, config) in build_map.iter() {
        extract_artifacts(dir_path, config, &project, &output.push())?;
    }

    output.step("[Release] - OK");
    Ok(())
}
