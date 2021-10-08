use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::{anyhow, Result};

use semver::Version;

fn load_from_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn set_package_version(manifest_path: &Path, new_version: &Version) -> Result<()> {
    let temp_manifest_path = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("Cargo.toml.work");

    {
        let manifest = load_from_file(manifest_path)?;
        let mut manifest: toml_edit::Document = manifest
            .parse()
            .map_err(|e| anyhow!("Couldn't parse Cargo.toml manifest: {}", e))?;
        manifest["package"]["version"] = toml_edit::value(format!("{}", new_version));

        let mut file_out = File::create(&temp_manifest_path)?;
        file_out.write_all(manifest.to_string().as_bytes())?;
    }
    fs::rename(temp_manifest_path, manifest_path)?;
    Ok(())
}

pub fn update_lock(manifest_path: &Path) -> Result<()> {
    cargo_metadata::MetadataCommand::new()
        .manifest_path(manifest_path)
        .exec()
        .map_err(|e| anyhow!("Couldn't update Cargo.lock: {}", e))?;

    Ok(())
}
