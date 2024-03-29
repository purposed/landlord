mod cli;

use clap::{App, AppSettings, Arg, SubCommand};

use rood::cli::OutputManager;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = App::new("lord")
        .version(VERSION)
        .author("Purposed")
        .about("Build & Release Facilitator")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("validate")
                .about("Builds & runs all validators for the current project")
                .arg(
                    Arg::with_name("project_path")
                        .long("path")
                        .required(false)
                        .help("The path of the project to validate")
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("mode")
                        .long("mode")
                        .possible_values(&["debug", "release"])
                        .default_value("release")
                        .help("Build mode"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short('v')
                        .help("Whether to use verbose output")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("release")
                .about("Bundles artifacts for release -- Fit for CI use")
                .arg(
                    Arg::with_name("project_path")
                        .long("path")
                        .required(false)
                        .help("The path of the project to publish")
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("mode")
                        .long("mode")
                        .possible_values(&["debug", "release"])
                        .default_value("release")
                        .help("Build mode"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short('v')
                        .help("Whether to use verbose output")
                        .required(false),
                )
                .arg(
                    Arg::with_name("nozip")
                        .long("nozip")
                        .help("Skip compression of the build artifacts.")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("publish")
                .about("Prepares a deployment from the local machine")
                .arg(
                    Arg::with_name("project_path")
                        .long("path")
                        .required(false)
                        .help("The path of the project to publish")
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Whether to use verbose output")
                        .required(false),
                )
                .arg(
                    Arg::with_name("dry")
                        .long("dry")
                        .help("Perform a dry run (no version bump or git tag)")
                        .required(false),
                )
                .arg(
                    Arg::with_name("unsafe")
                        .long("unsafe")
                        .hidden(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("mode")
                        .long("mode")
                        .possible_values(&["debug", "release"])
                        .default_value("release")
                        .help("Build mode"),
                )
                .arg(
                    Arg::with_name("level")
                        .possible_values(&["major", "minor", "patch", "none"])
                        .help("By how much to increment the version number.")
                        .required(true),
                ),
        );

    match cli::run_main(app.get_matches()) {
        Ok(_) => {}
        Err(e) => OutputManager::new(false).error(&format!("{}", e)),
    }
}
