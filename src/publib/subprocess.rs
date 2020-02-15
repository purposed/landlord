use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdout, Command, Stdio};

use rood::{Cause, CausedResult, Error};

pub fn run_cmd<T>(args: Vec<&str>, output_proc: T) -> CausedResult<String>
where
    T: Fn(&str),
{
    if args.is_empty() {
        return Err(Error::new(Cause::InvalidData, "No arguments to run_cmd"));
    }
    let mut cmd = Command::new(args.get(0).unwrap());
    let arg_cmd = cmd.args(&args[1..]);

    let mut child_process = arg_cmd
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let status = child_process.wait()?;
    if status.success() {
        let stdout = child_process.stdout.ok_or_else(|| {
            Error::new(
                Cause::GeneralError("SubprocessError".to_string()),
                "Could not attach to stdout",
            )
        })?;
        get_stdout(stdout)
    } else {
        let code = status.code().unwrap_or(1);
        get_stderr(child_process, output_proc)?;
        Err(Error::new(
            Cause::GeneralError("SubprocessError".to_string()),
            &format!("Status: {}", code),
        ))
    }
}

fn get_stdout(stdout: ChildStdout) -> CausedResult<String> {
    let total_vec: Vec<String> = BufReader::new(stdout)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    Ok(total_vec.join("\n").trim().to_string())
}

pub fn get_stderr<T>(c: Child, output_proc: T) -> CausedResult<()>
where
    T: Fn(&str),
{
    let stderr = c.stderr.ok_or_else(|| {
        Error::new(
            Cause::GeneralError("SubprocessError".to_string()),
            "Could not attach to stderr",
        )
    })?;

    BufReader::new(stderr)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| output_proc(&line));
    Ok(())
}
