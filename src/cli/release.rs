use std::fs;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

use publib::{BuildConfig, BuildMode, MetaBuilder, Project};

use rood::cli::OutputManager;
use rood::CausedResult;

use sha2::{Digest, Sha256};
use std::io::Write;

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

fn format_artifact_path(
    template: &str,
    config: &BuildConfig,
    project: &Project,
    build_dir: &str,
) -> String {
    template
        .replace("$(BUILD)", build_dir)
        .replace("$(PLATFORM)", &config.platform.value())
        .replace("$(ARCHITECTURE)", &config.architecture.value())
        .replace("$(PROJECT)", project.path.to_str().unwrap())
}

fn extract_artifacts(
    build_path: &PathBuf,
    config: &BuildConfig,
    project: &Project,
    output: &OutputManager,
) -> CausedResult<String> {
    output.step(&format!(
        "[Bundle/{}-{}]",
        config.platform, config.architecture
    ));

    let pushed = output.push();

    let artifact_dir = Path::new(&project.lease.artifact_directory)
        .join(&format!("{}-{}", config.platform, config.architecture));
    fs::create_dir_all(&artifact_dir)?;

    for artifact in project.lease.artifacts.iter() {
        let artifact_src_path = format_artifact_path(
            &artifact.path_template,
            config,
            project,
            build_path.to_str().unwrap(),
        );

        let artifact_dst_path;

        if let Some(dst_template) = &artifact.target_name_template {
            artifact_dst_path = artifact_dir.join(format_artifact_path(
                dst_template,
                config,
                project,
                build_path.to_str().unwrap(),
            ))
        } else {
            artifact_dst_path = artifact_dir.join(&artifact.name);
        }

        pushed.step(&format!(
            "{} ~> {}",
            artifact_src_path,
            artifact_dst_path.to_str().unwrap()
        ));

        if artifact.include_checksum {
            // Compute checksum of artifact.
            let checksum_file_path = artifact_dst_path.with_extension("sha256");
            pushed.debug(&format!(
                "Checksum file: {}",
                checksum_file_path.to_str().unwrap()
            ));
            let mut checksum = Sha256::new();
            let art_data = fs::read(&artifact_src_path)?;
            checksum.input(art_data);
            let checksum_value = checksum.result();
            fs::File::create(checksum_file_path)?.write_all(
                &format!(
                    "{:x}  {}\n",
                    checksum_value,
                    artifact_dst_path.file_name().unwrap().to_str().unwrap()
                )
                .as_bytes(),
            )?;
        }

        fs::copy(&artifact_src_path, artifact_dst_path)?;
    }

    output.step("[Bundle] - OK");
    Ok(String::from(artifact_dir.to_str().unwrap()))
}

pub fn release(matches: &ArgMatches) -> CausedResult<()> {
    let verbose = matches.is_present("verbose");
    let output = OutputManager::new(verbose);

    let mode = BuildMode::get(matches);

    let project_path = matches.value_of("project_path").unwrap(); // Mandatory argument.
    let project = publib::Project::new(project_path)?;

    output.step("[Release]");

    // Phase 1 - Ensure artifact dir exists.
    create_artifact_dir(&project.lease.artifact_directory, &output.push())?;

    // Phase 2 - Perform all builds.
    let metabuilder = MetaBuilder::default();
    let build_map = metabuilder.build(&project, mode, &output.push())?;

    for (dir_path, config) in build_map.iter() {
        let _artifact_dir = extract_artifacts(dir_path, config, &project, &output.push())?;
        // TODO: Compress.
    }

    output.step("[Release] - OK");
    Ok(())
}
