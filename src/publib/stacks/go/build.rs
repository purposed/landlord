use std::env;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use rood::sys::file;

use crate::{subprocess, BuildConfig, BuildMode, Builder, Project};

#[derive(Default)]
pub struct GoBuilder {}

impl GoBuilder {
    pub fn new() -> GoBuilder {
        GoBuilder {}
    }

    fn get_module_name(&self, project: &Project) -> Result<String> {
        let gomod_path = project.path.join("go.mod");
        file::ensure_exists(&gomod_path)?;

        let gomod_raw = fs::read_to_string(&gomod_path)?;
        let mod_def: Option<&str> = gomod_raw.split('\n').next();

        if let Some(mod_name) = mod_def {
            Ok(mod_name
                .split(' ')
                .last()
                .ok_or_else(|| anyhow!("Unexpected go.mod format"))?
                .to_string())
        } else {
            Err(anyhow!("Unexpected go.mod format"))
        }
    }
}

impl Builder for GoBuilder {
    fn build(
        &self,
        project: &Project,
        config: &BuildConfig,
        _mode: &BuildMode, // TODO: Use.
    ) -> Result<PathBuf> {
        let architecture = {
            let archs = config.architecture.value();

            archs
                .get(0)
                .ok_or_else(|| anyhow!("Invalid architecture"))?
                .clone()
        };

        let platform = config.platform.value().first().unwrap().clone();

        let path = project.path.clone();
        env::set_var("GOOS", &platform);
        env::set_var("GOARCH", &architecture);

        let out_dir = path
            .join("bin")
            .join("release")
            .join(format!("{}-{}", platform, &architecture));

        let target_name: &str;
        if let Some(t_name) = &config.name {
            target_name = t_name;
        } else {
            target_name = &project.lease.name
        }

        let artifact_path = out_dir.join(target_name);

        let module_name = self.get_module_name(project)?;

        let mut commit_flag = project.repository.short_head()?;
        if project.repository.has_uncommitted_changes()? {
            commit_flag = commit_flag.add("-dirty")
        }

        let ldflags = format!(
            "-w -X {}/version.VERSION={} -X {}/version.GITCOMMIT={} -extldflags -static",
            module_name, project.lease.version, module_name, commit_flag
        );

        let mut cmd = vec![
            "go",
            "build",
            "-o",
            artifact_path.to_str().unwrap(),
            "-a",
            "-tags",
            "static_build netgo",
            "-installsuffix",
            "netgo",
            "-ldflags",
            &ldflags,
        ];
        if let Some(p) = &config.src_path {
            cmd.push(p);
        } else {
            cmd.push(".");
        }

        subprocess::run_cmd(cmd, |l| eprintln!("{}", l))?;

        env::remove_var("GOOS");
        env::remove_var("GOARCH");

        Ok(out_dir)
    }

    fn clean(&self) -> Result<()> {
        unimplemented!();
    }
}
