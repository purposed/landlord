use crate::subprocess;

use rood::{Cause, CausedResult, Error};
use std::path::{Path, PathBuf};

use super::Dir;

#[derive(Debug)]
pub struct Repository {
    path: PathBuf,
}

impl Repository {
    pub fn open<T>(path: T) -> CausedResult<Repository>
    where
        T: AsRef<Path>,
    {
        let r = Repository {
            path: PathBuf::from(path.as_ref()),
        };

        r.ensure_exists()?;

        Ok(r)
    }

    pub fn short_head(&self) -> CausedResult<String> {
        let _moved = Dir::move_to(&self.path)?;
        subprocess::run_cmd(vec!["git", "rev-parse", "--short", "HEAD"], |_l| {})
    }

    pub fn ensure_exists(&self) -> CausedResult<()> {
        let _moved = Dir::move_to(&self.path)?;

        let output =
            subprocess::run_cmd(vec!["git", "rev-parse", "--is-inside-work-tree"], |_l| {})?;
        if &output != "true" {
            return Err(Error::new(Cause::InvalidState, "Not in a git repository"));
        }

        Ok(())
    }

    pub fn has_uncommitted_changes(&self) -> CausedResult<bool> {
        let _moved = Dir::move_to(&self.path)?;
        let output = subprocess::run_cmd(
            vec!["git", "status", "--porcelain", "--untracked-files=no"],
            |_l| {},
        )?;

        Ok(!output.is_empty())
    }

    pub fn current_branch(&self) -> CausedResult<String> {
        let _moved = Dir::move_to(&self.path)?;
        subprocess::run_cmd(vec!["git", "rev-parse", "--abbrev-ref", "HEAD"], |_l| {})
    }

    pub fn commit_all(&self, message: &str) -> CausedResult<()> {
        let _moved = Dir::move_to(&self.path);

        // Stage all changes.
        subprocess::run_cmd(vec!["git", "add", "."], |_l| {})?;

        let _output =
            subprocess::run_cmd(vec!["git", "commit", "-m", message], |l| eprintln!("{}", l))?;
        Ok(())
    }

    pub fn push(&self, remote: &str, target: &str) -> CausedResult<()> {
        let _moved = Dir::move_to(&self.path);
        let _output =
            subprocess::run_cmd(vec!["git", "push", remote, target], |l| eprintln!("{}", l))?;
        Ok(())
    }

    pub fn add_tag(&self, tag: &str) -> CausedResult<()> {
        let _moved = Dir::move_to(&self.path);
        subprocess::run_cmd(vec!["git", "tag", tag], |_l| {})?;
        Ok(())
    }
}
