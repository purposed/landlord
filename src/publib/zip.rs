use rood::CausedResult;

use super::subprocess;

#[cfg(target_family = "unix")]
fn zip_unix(zip_path: &str, files_in_dir: &[String]) -> CausedResult<()> {
    let mut cmds = vec!["zip", "-r", zip_path];
    files_in_dir.iter().for_each(|f| cmds.push(f));
    subprocess::run_cmd(cmds, |_l| {})?;

    Ok(())
}

pub fn zip_directory(zip_path: &str, files_in_dir: &[String]) -> CausedResult<()> {
    if cfg!(unix) {
        zip_unix(zip_path, files_in_dir)
    } else {
        // TODO: Handle other platforms.
        Ok(())
    }
}
