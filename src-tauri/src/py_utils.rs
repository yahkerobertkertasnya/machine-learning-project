use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Child, Command, Stdio};

use crate::constants::{CREATE_NO_WINDOW_FLAGS, VENV_DIRECTORY};

pub fn run_script(script_path: &Path, args: Vec<String>) -> Child {
    let venv_path = if cfg!(windows) {
        Path::new(VENV_DIRECTORY)
            .join("Scripts")
            .join("activate.bat")
    } else {
        Path::new(VENV_DIRECTORY).join("bin").join("activate")
    };

    let activate_script = venv_path.to_str().unwrap();

    let mut values = Vec::new();

    values.extend(args.iter().map(|s| s.to_string()));

    let string_path = script_path.to_str().unwrap();

    let mut command_args = vec!["/C", &activate_script, "&&", "python", string_path];

    command_args.extend(values.iter().map(|s| s.as_str()));

    Command::new("cmd")
        .args(&command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW_FLAGS)
        .spawn()
        .map_err(|e| format!("Failed to execute Python script: {}", e))
        .unwrap()
}
