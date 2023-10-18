use crate::config::Config;
use std::process::{Command, Stdio};

// Take the text from a file, decrypts it and return the text. Does **NOT** decrypt the file on disk
pub fn decr(filepath: String) -> Result<String, String> {
    match Command::new("gpg")
        // decrypt file quietly 
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

// Take the text from a file, encrypt it and return the text. Does **NOT** encrypt the file on disk
pub fn encr_and_output(filepath: String, config: &Config) -> Result<String, String> {
    match Command::new("gpg")
        .args([
            // armored encryption
            "-ea",
            // output to stdout
            "--output",
            "-",
            // don't show confirmation prompts of unsigned keys
            "--trust-model",
            "always",
            // auto assume yes for any questions
            "--batch",
            "--yes",
            // Make sure we use the the right GPG key
            "--recipient",
            config.public_gpg_key.as_str(),
            &filepath,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
    {
        Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
        Err(err) => return Err(err.to_string()),
    };
}

// Encrypt a string and return it
pub fn encr_string_and_output(input: String, config: &Config) -> Result<String, String> {
    let cmd = Command::new("echo")
        .arg(&input)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    match Command::new("gpg")
        .args([
            // armored encryption
            "-ea",
            // output to stdout
            "--output",
            "-",
            // don't show confirmation prompts of unsigned keys 
            "--trust-model",
            "always",
            // auto assume yes for any questions
            "--batch",
            "--yes",
            // Make sure we use the right GPG key
            "--recipient",
            config.public_gpg_key.as_str(),
        ])
        // take the stdout from echo, as the stdin for gpg
        .stdin(cmd.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
    {
        Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
        Err(err) => return Err(err.to_string()),
    };
}
