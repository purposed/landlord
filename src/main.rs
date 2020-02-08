use std::env;
use std::io;

use clap::{App, AppSettings, Arg, SubCommand};

mod cli;

use publib;
use rood::cli::OutputManager;

fn main() {
    let app = App::new("lord")
        .version("0.1.0")
        .author("Purposed")
        .about("Build & Release Facilitator")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds a project")
                .arg(
                    Arg::with_name("project_path")
                        .required(false)
                        .help("The path of the project to build")
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Whether to use verbose output")
                        .required(false),
                )
                .arg(
                    Arg::with_name("debug")
                        .long("debug")
                        .help("Build in debug mode")
                        .conflicts_with("release"),
                )
                .arg(
                    Arg::with_name("release")
                        .long("release")
                        .help("Build in release mode")
                        .conflicts_with("debug"),
                ),
        );

    match cli::run_main(app.get_matches()) {
        Ok(_) => {}
        Err(e) => OutputManager::new(false).error(&format!("{}", e)),
    }
}
