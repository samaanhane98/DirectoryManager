# DirectoryManager

Directory File Management Tool Written In Rust

## Install

```bash
git clone git@github.com:samaanhane98/DirectoryManager.git
cd DirectoryManager

cargo build --release

mv ./target/release/directory-manager /usr/local/bin/directory-manager
mv ./Config.toml /usr/local/bin/Config.toml
```

## Add Cron

On Arch, using cronie

```
crontab -e
```

Cron

```cron
* * * * * /usr/local/bin/directory-manager
```

## Usage

The following is an example of a `config.toml` which must be added to the managed directory:

```
[extensions]
pdf = "PDF"
png = "Images"
```

This will move a file ending in the extension to the corresponding directory. If the directory does not exist the program creates it first.

Add directories you want to manage to the program its `Config.toml` as follows:

```toml
[directories.{NAME}]
config = '{PATH TO DIRECTORY CONFIG}'
```
