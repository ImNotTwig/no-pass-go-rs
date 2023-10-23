# no-pass-go-rs
no-pass-go (npg) is a password/account manager written in Rust, very much inspired by [pass](https://www.passwordstore.org/), the name however is a reference to the Monopoly phrase "Do not pass Go and do not collect $200". 

With npg you have three _essential_ commands:
- show
- add
- remove

Other commmands are provided, but the above commands are all you need to use this program.

Filepath arguments given to this program should be relative to the account store folder if the filepath to the account file is `passwords/email/google.com/username`, then you only need to provide `email/google.com/username` as an argument.

- `npg show` shows the password, and optionally the metadata of a given account filepath. e.g: `email/google.com/username`
- `npg add` adds an account to the database, it requires a password to be defined. All the fields are defined through flags, including the password. a filepath is required.
- `npg remove` removes an account from the database, you only need to provide the filepath.

There are 3 major features of this program
1. It stores metadata for you, including usernames, emails, and service names (websites). It not only stores it for you, its stored in json, (encrypted of course)
2. The accounts are stored individually in their own files, which are hashes of the filepath given, the filepaths are stored in an index file called `pass_tree.asc`, each filepath has its own line, and is formatted as `path/to/account_file:hash_of_filepath`. The file is of course encrypted, as to not leak metadata
3. You can easily integrate this program into outside programs or scripts, because of the way that data is printed with the `show` command, through normal plaintext messages, or json

You need the rust compiler to run this program. You can find out how to install Rust on your system by using your favorite search engine and searching "how to install Rust on <operating_system>"
A universal install guide is available [here](https://rustup.rs/#)

Make sure you rename `example_config.toml` to `config.toml`, and put in your **public** GPG key, and the absolute path to where you want to store your account data.
After making renaming and putting in your gpg key and account store directory in `config.toml`, move it to `~/.config/npg/config.toml`
> The `BaseDirectory` field needs an absolute path e.g: `/home/user/passwords` **NOT** `~/passwords`

Here is a [guide](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/security_guide/sect-security_guide-encryption-gpg-creating_gpg_keys_using_the_command_line) on how to make a GPG key

to build and install the program after installing Rust and setting up your config file:
```sh
git clone https://github.com/ImNotTwig/no-pass-go-rs
cd no-pass-go-rs
cargo build --release
cp target/release/npg .local/bin/npg
```

The `rofi-npg.sh` script is a script to incorporate this program (npg) with a run launcher (rofi, wofi, fuzzel, dmenu, etc). This script is HARDCODED with `fuzzel`, `doas ydotool`, and my path to the `npg` binary.
You need to replace the references to these with your own run launcher, input automation tool, and a path to the binary for `npg`
