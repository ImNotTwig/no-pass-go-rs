# Password Manager (npg)

No Pass Go is a password/account manager written in rust, inspired by [pass](https://www.passwordstore.org)
I jokingly call npg the standard post-Unix password manager, because as I see it, its an extension of the concept of pass, and this fixes a few issues of pass that I saw.
The name is a reference to the Monopoly board game phrase: "Do not pass Go; do not collect $200."

> Filepath arguments given to this program should be relative to the account store folder.
> If the filepath to the account file is `~/.passwords/email/google.com/username`, then you only need to provide `email/google.com/username` as an argument.


### With `npg` you have three _essential_ commands:

> Other commands are provided, but the below commands are all you _need_ to use this program.
- `npg show` shows the password, and optionally the metadata of a given account store folder. e.g. `email/google.com/username` as an argument.
- `npg add` adds an account to the database, it requires a password and a filepath to be defined.
- `npg remove` removes an account from the database, you only need to provide the filepath.


### Some features of npg

- It stores metadata for you, including usernames, emails, and service names (websites).
- Under the hood, the metadata/password is stored in json before its encrypted.
- The accounts are stored individually in their own files, which are named as hashes of the filepath given.
- The filepaths are stored in an encrypted index file called `pass_tree.asc`. Each filepath has its own line, and is formatted as `path/to/account_file:hash_of_filepath`.
- You can easily integrate this program into outside programs or scripts, because of the way that data is printed with the `show` and `list` command.
> You need the rust compiler to run this program.
> A universal install guide is available [here](https://rustup.rs/#).

Make sure you rename `example_config.toml` to `config.toml` and fill it out.
After making renaming and putting in your GPG key and account store directory in `config.toml`, move it to `~/.config/npg/config.toml`
> The BaseDirectory field needs to be an absolute path e.g. `/home/user/.passwords` **NOT** `~/.passwords`

I recommend installing this program to `~/.local/bin`, as shown below
```bash
git clone https://github.com/ImNotTwig/no-pass-go-rs
cd no-pass-go-rs
cargo build --release
cp ./target/release/npg ~/.local/bin/
```
