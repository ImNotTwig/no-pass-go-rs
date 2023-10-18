use accounts::Account;
use clap::{Args, Parser, Subcommand};
use config::get_config;
use files::*;

mod accounts;
mod config;
mod crypto;
mod files;

#[derive(Parser, Debug)]
#[command(
    about = r"A Password manager written in Rust, storing in encrypted json, following the post-unix philopshy."
)]
#[command(name = "npg")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Adds an account to the store")]
    Add(Add),
    #[command(visible_alias = "rm")]
    #[command(about = "Removes an account from the store")]
    Remove(Remove),
    #[command(about = "Show an account from the store")]
    Show(Show),
    #[command(about = "Change the filepath of an account in the store")]
    #[command(visible_alias = "mv")]
    Move(Move),
    #[command(about = "Edit an account in the store")]
    #[command(visible_alias = "ed")]
    Edit(Edit),
    #[command(about = "List the accounts from the store")]
    #[command(visible_alias = "ls")]
    List(List),
}

#[derive(Args, Debug)]
struct Add {
    #[arg(short = 'p', long = "password", value_name = "password")]
    password: String,

    #[arg(short = 'u', long = "username", value_name = "username")]
    username: Option<String>,

    #[arg(short = 'e', long = "email", value_name = "email")]
    email: Option<String>,

    #[arg(short = 's', long = "service", value_name = "service")]
    service: Option<String>,

    path: String,
}

#[derive(Args, Debug)]
struct Remove {
    path: String,
}

#[derive(Args, Debug)]
struct Show {
    #[arg(short = 'a', long = "all")]
    all: bool,

    path: String,
}

#[derive(Args, Debug)]
struct Move {
    old_path: String,
    new_path: String,
}

#[derive(Args, Debug)]
struct Edit {
    path: String,
}

#[derive(Args, Debug)]
struct List {}

fn main() {
    let cli = Cli::parse();

    let mut config = get_config();

    config.base_directory = config.base_directory.trim_matches('"').to_string();
    config.public_gpg_key = config.public_gpg_key.trim_matches('"').to_string();

    match &cli.command {
        // Add Subcommand
        Some(Commands::Add(account)) => {
            match add_account(
                &config,
                Account {
                    password: account.password.to_owned(),
                    username: account.username.to_owned(),
                    email: account.email.to_owned(),
                    service: account.service.to_owned(),
                },
                account.path.to_owned(),
            ) {
                Err(err) => {
                    panic!("{}", err);
                }
                Ok(_) => {
                    match add_to_tree_file(&config, account.path.to_owned()) {
                        Ok(_) => {}
                        Err(err) => {
                            panic!("{}", err);
                        }
                    }
                    println!("Added {} to the store.", account.path);
                }
            }
        }

        // Remove Subcommand
        Some(Commands::Remove(account_path)) => {
            match remove_account(&config, account_path.path.to_owned()) {
                Err(err) => {
                    panic!("{}", err);
                }
                Ok(_) => {
                    match remove_from_tree_file(&config, account_path.path.to_owned()) {
                        Ok(_) => {}
                        Err(err) => panic!("{}", err),
                    }
                    println!("Removed {} from the store.", account_path.path.to_owned());
                }
            }
        }

        // Show Subcommand
        Some(Commands::Show(args)) => match open_account(&config, args.path.to_owned()) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(account) => {
                println!("{}", account.password);
                if args.all == true {
                    println!("username: {}", account.username.unwrap_or("".to_string()));
                    println!("email: {}", account.email.unwrap_or("".to_string()));
                    println!("service: {}", account.service.unwrap_or("".to_string()));
                }
            }
        },

        // Edit Subcommand
        Some(Commands::Edit(account_path)) => {
            match edit_account(&config, account_path.path.to_owned()) {
                Err(err) => {
                    panic!("{}", err);
                }
                Ok(_) => {
                    println!("Saved edits to {}", account_path.path);
                }
            }
        }

        // Move Subcommand
        Some(Commands::Move(paths)) => {
            match move_account(
                &config,
                paths.old_path.to_owned(),
                paths.new_path.to_owned(),
            ) {
                Err(err) => {
                    panic!("{}", err);
                }
                Ok(_) => {
                    println!("Moved {} to {}", paths.old_path, paths.new_path)
                }
            }
        }

        // List Subcommand
        Some(Commands::List(_)) => match list_accounts(&config) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(path_string) => {
                for string in path_string.split("\n") {
                    let path: Vec<&str> = string.split(":").collect();
                    if path[0] == "" {
                        continue;
                    }
                    println!("{}", path[0]);
                }
            }
        },
        _ => {}
    }
}
