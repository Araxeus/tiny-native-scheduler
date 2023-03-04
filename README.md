# tiny native scheduler

-   This is a tiny native scheduler for the [Rust](https://www.rust-lang.org/) programming language.
    It uses [schtasks](https://technet.microsoft.com/en-us/library/cc725744.aspx) to schedule tasks on Windows. and [at](<https://en.wikipedia.org/wiki/At_(Unix)>) on Unix.
    
> * Windows: always work since `schtasks` is always available
> * Linux: work if `at` is installed
> * macOS: almost never work since `at` isn't really supported there

## Installation

```bash
cargo install tiny_scheduler
```
Or add this to your `Cargo.toml`:

```toml
[dependencies]
tiny_scheduler = "0.1.0"
```

## Usage

```rust
use tiny_scheduler::execute_command_in_x_minutes;

fn main() {
    execute_command_in_x_minutes("cargo install tiny_scheduler", 2);
}
```

## Documentation

### `execute_command_in_x_minutes`

Execute a command in x minutes using `at` or `schtasks` depending on the OS.

#### Arguments

-   `command` - The command to execute.
-   `minutes` - The amount of minutes to wait before executing the command.
-   `win_task_name` - The name of the task to create on Windows.

#### Example

```rust
use execute_command_in_x_minutes::execute_command_in_x_minutes;

execute_command_in_x_minutes("cargo install cargo-update", 5).unwrap();
```

#### Errors

This function will return an error if the command fails to execute.
