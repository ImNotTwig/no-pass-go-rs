use crate::config::Config;
use std::process::{Command, Stdio};

pub fn decr(filepath: String) -> Result<String, String> {
    match Command::new("gpg")
        .args(["-dq", &format!("{}", filepath)])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("There was an error while calling gpg")
        .wait_with_output()
    {
        Ok(command) => return Ok(String::from_utf8(command.stdout).unwrap()),
        Err(err) => return Err(err.to_string()),
    }
}

pub fn encr_and_output(filepath: String, config: &Config) -> Result<String, String> {
    match Command::new("gpg")
        .args([
            "-ea",
            "--output",
            "-",
            "--trust-model",
            "always",
            "--batch",
            "--yes",
            "--recipient",
            config.public_gpg_key.as_str(),
            &filepath,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
    {
        Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
        Err(err) => return Err(err.to_string()),
    };
}

pub fn encr_string_and_output(input: String, config: &Config) -> Result<String, String> {
    let cmd = Command::new("echo")
        .arg(&input)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    match Command::new("gpg")
        .args([
            "-ea",
            "--output",
            "-",
            "--trust-model",
            "always",
            "--batch",
            "--yes",
            "--recipient",
            &format!("{}", config.public_gpg_key.as_str()),
        ])
        .stdin(cmd.stdout.unwrap()) // Pipe through.
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
    {
        Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
        Err(err) => return Err(err.to_string()),
    };
}
