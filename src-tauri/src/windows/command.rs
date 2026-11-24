use std::io::Read;
use std::os::windows::process::CommandExt;
use std::process::{Child, Command, Output, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const POLL_INTERVAL: Duration = Duration::from_millis(25);
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);

const POWERSHELL_PRELUDE: &str = "$ProgressPreference = 'SilentlyContinue'; [Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false); ";

fn read_pipe(pipe: Option<impl Read + Send + 'static>) -> JoinHandle<std::io::Result<Vec<u8>>> {
    thread::spawn(move || {
        let mut buffer = Vec::new();
        if let Some(mut pipe) = pipe {
            pipe.read_to_end(&mut buffer)?;
        }
        Ok(buffer)
    })
}

fn collect_pipe(
    program: &str,
    reader: JoinHandle<std::io::Result<Vec<u8>>>,
) -> Result<Vec<u8>, String> {
    reader
        .join()
        .map_err(|_| format!("Could not read output from {program}"))?
        .map_err(|error| format!("Could not read output from {program}: {error}"))
}

fn wait_with_timeout(
    program: &str,
    child: &mut Child,
    timeout: Duration,
) -> Result<std::process::ExitStatus, String> {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if Instant::now() >= deadline => {
                child
                    .kill()
                    .and_then(|()| child.wait().map(|_| ()))
                    .map_err(|error| format!("Could not stop {program} after timeout: {error}"))?;
                return Err(format!(
                    "{program} did not finish within {} seconds",
                    timeout.as_secs()
                ));
            }
            Ok(None) => thread::sleep(POLL_INTERVAL),
            Err(error) => return Err(format!("Could not wait for {program}: {error}")),
        }
    }
}

pub fn run_hidden(program: &str, args: &[&str], timeout: Duration) -> Result<Output, String> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|error| format!("Could not run {program}: {error}"))?;
    let stdout = read_pipe(child.stdout.take());
    let stderr = read_pipe(child.stderr.take());
    let status = wait_with_timeout(program, &mut child, timeout)?;
    Ok(Output {
        status,
        stdout: collect_pipe(program, stdout)?,
        stderr: collect_pipe(program, stderr)?,
    })
}

pub fn run_powershell(script: &str, timeout: Duration) -> Result<String, String> {
    let script = format!("{POWERSHELL_PRELUDE}{script}");
    let output = run_hidden(
        "powershell.exe",
        &[
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ],
        timeout,
    )?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        return Err(if message.is_empty() {
            format!("PowerShell exited with {}", output.status)
        } else {
            message
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub fn run_powershell_json<T: serde::de::DeserializeOwned>(
    script: &str,
    timeout: Duration,
) -> Result<T, String> {
    let output = run_powershell(script, timeout)?;
    serde_json::from_str(&output).map_err(|error| format!("Invalid PowerShell response: {error}"))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{DEFAULT_TIMEOUT, run_hidden, run_powershell};

    #[test]
    fn powershell_output_is_utf8() {
        let output = run_powershell("'Größe ✓'", DEFAULT_TIMEOUT).unwrap();

        assert_eq!(output, "Größe ✓");
    }

    #[test]
    fn stops_commands_that_exceed_the_timeout() {
        let result = run_hidden(
            "powershell.exe",
            &["-NoProfile", "-Command", "Start-Sleep -Seconds 20"],
            Duration::from_millis(500),
        );

        assert!(result.unwrap_err().contains("did not finish"));
    }
}
