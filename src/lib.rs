use std::{
    io::{Error, ErrorKind},
    process::{Command, Output, Stdio},
};

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
    win_task_name: &str,
) -> CustomResult<Output> {
    let output = if cfg!(windows) {
        use time::{format_description, Duration, OffsetDateTime};

        let scheduled_time = OffsetDateTime::now_local()? + Duration::minutes(minutes);
        let format = format_description::parse("[hour]:[minute]")?;

        let scheduled_time_string = scheduled_time.format(&format)?;

        Command::new("schtasks")
            .args([
                "/create",
                "/tn",
                win_task_name,
                "/tr",
                &format!("cmd /C start \"\" /MIN \"cmd\" \"/C {command}\""),
                "/sc",
                "once",
                "/st",
                &scheduled_time_string,
                "/f",
            ])
            .output()?
    } else {
        let output = Command::new("echo")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "'at' failed to read from 'echo'"))?;

        Command::new("at")
            .args(["now", "+", &minutes.to_string(), "minute"])
            .stdin(Stdio::from(output))
            .output()?
    };

    Ok(output)
}
