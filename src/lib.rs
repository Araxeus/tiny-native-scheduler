use std::process::{Command, Output};

type CustomResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Execute a command in x minutes using `at` or `schtasks` depending on the OS.
///
/// Returns the output of the command.
///
/// # Arguments
///
/// * `command` - The command to execute.
/// * `minutes` - The amount of minutes to wait before executing the command.
/// * `win_task_name` - The name of the task to create on Windows.
///
/// # Example
///
/// ```no_run
/// use execute_command_in_x_minutes::execute_command_in_x_minutes;
///
/// execute_command_in_x_minutes("cargo install cargo-update", 5).unwrap();
/// ```
///
/// # Errors
///
/// This function will return an error if the command fails to execute.
///
pub fn execute_command_in_x_minutes(
    command: &str,
    minutes: i64,
    #[allow(unused_variables)] win_task_name: &str,
) -> CustomResult<Output> {
    #[cfg(windows)]
    let output = exec_in_x_minutes_win(command, minutes, win_task_name)?;
    #[cfg(not(windows))]
    let output = exec_in_x_minutes_unix(command, minutes)?;

    Ok(output)
}

#[cfg(windows)]
use time::{format_description, Duration, OffsetDateTime};

#[cfg(windows)]
fn exec_in_x_minutes_win(command: &str, minutes: i64, task_name: &str) -> CustomResult<Output> {
    let scheduled_time = OffsetDateTime::now_local()? + Duration::minutes(minutes);
    let format = format_description::parse("[hour]:[minute]")?;

    let scheduled_time_string = scheduled_time.format(&format)?;

    let output = Command::new("schtasks")
        .args([
            "/create",
            "/tn",
            task_name,
            "/tr",
            &format!("cmd /C start \"\" /MIN \"cmd\" \"/C {command}\""),
            "/sc",
            "once",
            "/st",
            &scheduled_time_string,
            "/f",
        ])
        .output()?;

    Ok(output)
}

#[cfg(not(windows))]
use std::{
    io::{Error, ErrorKind},
    process::Stdio,
};

#[cfg(not(windows))]
fn exec_in_x_minutes_unix(command: &str, minutes: i64) -> CustomResult<Output> {
    let output = Command::new("echo")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "'at' failed to read from 'echo'"))?;

    let output = Command::new("at")
        .args(["now", "+", &minutes.to_string(), "minute"])
        .stdin(Stdio::from(output))
        .output()?;

    Ok(output)
}
