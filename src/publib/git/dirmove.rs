use std::env;
use std::path::{Path, PathBuf};

use rood::CausedResult;

pub struct Dir {
    return_path: PathBuf,
}

// TODO: Generalize this whole deferred struct into either a defer object or a defer!() macro.
impl Dir {
    pub fn move_to<T>(path: T) -> CausedResult<Dir>
    where
        T: AsRef<Path>,
    {
        let return_path = env::current_dir()?;
        env::set_current_dir(&path)?;
        Ok(Dir { return_path })
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        env::set_current_dir(&self.return_path).unwrap();
    }
}
