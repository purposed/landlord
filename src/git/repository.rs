use publib::subprocess;
use rood::CausedResult;
use std::env;
use std::intrinsics::uninit;
use std::path::{Path, PathBuf};

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

    pub fn ensure_exists(&self) -> CausedResult<()> {
        let cur_dir = env::current_dir()?;

        env::set_current_dir(&self.path)?;
        subprocess::run_cmd(vec!["git", "rev-parse", "--is-inside-work-tree"], |_l| {})?;
        env::set_current_dir(&cur_dir)?;

        Ok(())
    }

    pub fn has_uncommitted_changes(&self) -> CausedResult<bool> {
        unimplemented!();
        Ok(false)
    }

    pub fn current_branch(&self) -> CausedResult<String> {
        unimplemented!();
        Ok(String::from("master"))
    }
}
