use crate::{accounts::Account, config::Config, crypto::*};
use serde_json;
use sha256::digest;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

type TreeDataBase = HashMap<String, String>;

fn open_and_truncate_file(filepath: String) -> Result<File, String> {
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filepath.to_owned())
    {
        Ok(res) => return Ok(res),
        Err(err) => return Err(err.to_string()),
    };
}

fn open_file(filepath: String) -> Result<File, String> {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .open(filepath.to_owned())
    {
        Ok(res) => return Ok(res),
        Err(err) => return Err(err.to_string()),
    }
}

// Open and parse the tree database file (<BaseDirectory>/pass_tree.asc)
fn parse_tree_file(config: &Config) -> Result<TreeDataBase, String> {
    // if the tree database file does not exist, create it, and write an encrypted empty string to it
    if !Path::new(&format!("{}/pass_tree.asc", config.base_directory)).exists() {
        let mut pass_tree_file = open_file(format!("{}/pass_tree.asc", config.base_directory))?;

        let encr_nothing = encr_string_and_output("".to_string(), &config).unwrap();

        let _ = pass_tree_file.write(encr_nothing.as_bytes());
    }

    let tree_database = decr(format!("{}/pass_tree.asc", config.base_directory))?
        .trim()
        .to_string();

    let mut treedb = TreeDataBase::new();
    // for every line, parse the line into an entry in the TreeDataBase HashMap
    // The lines are formatted as "<account_path>:<account_path_hash>"
    for string in tree_database.split('\n') {
        if string == "" {
            continue;
        }
        let path_and_hash: Vec<&str> = string.split(":").collect();
        treedb.insert(path_and_hash[1].to_string(), path_and_hash[0].to_string());
    }

    return Ok(treedb);
}

// Write a TreeDataBase HashMap to the tree database file (<BaseDirectory>/pass_tree.asc)
fn write_tree_file(config: &Config, treedb: TreeDataBase) -> Result<(), String> {
    // Collect all the entries in the TreeDataBase HashMap into a single multiline string
    let mut treedb_string = String::new();
    for (hash, path) in &treedb {
        treedb_string += &(path.to_string() + ":" + hash);
    }

    let mut tree_file = open_and_truncate_file(format!("{}/pass_tree.asc", config.base_directory))?;

    let encr_treedb = encr_string_and_output(treedb_string.trim().to_string(), &config).unwrap();

    match tree_file.write(encr_treedb.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

// Remove an entry from the tree database file (<BaseDirectory>/pass_tree.asc)
pub fn remove_from_tree_file(config: &Config, path: String) -> Result<(), String> {
    let mut treedb = parse_tree_file(&config)?;
    treedb.remove(&digest(path));
    write_tree_file(&config, treedb)?;
    Ok(())
}

// Add an entry to the tree database file (<BaseDirectory>/pass_tree.asc)
pub fn add_to_tree_file(config: &Config, path: String) -> Result<(), String> {
    let mut treedb = parse_tree_file(&config)?;
    treedb.insert(digest(&path), path);
    println!("{:?}", treedb);
    write_tree_file(&config, treedb)?;
    Ok(())
}

// return a multiline string of the plaintext tree database file (<BaseDirectory>/pass_tree.asc)
pub fn list_accounts(config: &Config) -> Result<String, String> {
    let treedb = parse_tree_file(&config)?;
    let mut treedb_string = String::new();
    for (hash, path) in treedb {
        treedb_string += &format!("{}:{}\n", path, hash);
    }

    Ok(treedb_string)
}

// Write a new account to file
pub fn create_account_file(config: &Config, account: Account, account_path: String) -> Result<(), String> {
    let account_path = digest(account_path);

    let mut account_file =
        open_and_truncate_file(format!("{}/{}", config.base_directory, account_path))?;

    let account_string =
        serde_json::to_string(&account).expect("Could not parse account into json");
    let encr_account_string = encr_string_and_output(account_string, &config).unwrap();

    match account_file.write(&encr_account_string.trim().as_bytes()) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }
    Ok(())
}

// Remove an account from file
pub fn remove_account(config: &Config, account_path: String) -> Result<(), String> {
    let account_path = digest(account_path);
    open_and_truncate_file(format!("{}/{}", config.base_directory, account_path))?;

    match fs::remove_file(format!("{}/{}", config.base_directory, account_path)) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

// Move an account from one file to another
pub fn move_account(
    config: &Config,
    old_account_path: String,
    new_account_path: String,
) -> Result<(), String> {
    if let Ok(account) = open_account(config, old_account_path.to_owned()) {
        create_account_file(config.to_owned(), account, new_account_path.to_owned())?;
        add_to_tree_file(config, new_account_path)?;

        remove_account(config.to_owned(), old_account_path.to_owned())?;
        remove_from_tree_file(config.to_owned(), old_account_path)?;
    };
    Ok(())
}

// Edit an account in-place
pub fn edit_account(config: &Config, account_path: String) -> Result<(), String> {
    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(err) => return Err(err.to_string()),
    };

    // Write the pre edited account to a temporary file
    let pre_edit_account = open_account(config, account_path.to_owned())?;
    create_account_file(config, pre_edit_account, "temp".to_string())?;

    let hashed_temp = format!("{}/{}", config.base_directory, digest("temp".to_string()));
    let decr_temp = decr(hashed_temp.to_owned())?;

    let mut temp_file = open_and_truncate_file(hashed_temp.to_owned())?;

    match temp_file.write(decr_temp.as_bytes()) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    // Open the temp file in $EDITOR
    _ = Command::new(editor.to_owned())
        .arg(hashed_temp.to_owned())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    let encr_temp = encr_and_output(hashed_temp.to_owned(), config.to_owned())?;

    let mut temp_file = open_and_truncate_file(hashed_temp.to_owned())?;

    match temp_file.write(encr_temp.as_bytes()) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    let temp_account = open_account(config.to_owned(), "temp".to_string()).unwrap();
    create_account_file(config.to_owned(), temp_account, account_path)?;

    match fs::remove_file(hashed_temp) {
        Ok(_) => {},
        Err(err) => return Err(err.to_string()),
    }

    Ok(())
}

// Open and return an account from file
pub fn open_account(config: &Config, account_path: String) -> Result<Account, String> {
    let account_path = format!("{}/{}", config.base_directory, digest(account_path));

    if !Path::new(account_path.as_str()).exists() {
        return Err("Account file does not exist".to_string());
    }

    let account = decr(account_path)?;

    match serde_json::from_str::<Account>(&account) {
        Ok(account) => return Ok(account),
        Err(err) => return Err(err.to_string()),
    }
}
