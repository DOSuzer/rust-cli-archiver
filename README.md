# rust-cli-archiver
```
Usage: rust-cli-archiver [OPTIONS] [COMMAND]

Commands:
  extract  Extract archive to current location
  help     Print this message or the help of the given subcommand(s)

Options:
  -f, --files <FILES>  Path to file to be archived
  -n, --name <NAME>    Archive name with extension
  -h, --help           Print help
  -V, --version        Print version

Extract archive to current location

Usage: rust-cli-archiver.exe extract [OPTIONS]

Options:
  -f, --file <FILE>                Path to archive
  -d, --destination <DESTINATION>  Path where to extract (Work in progress)
  -h, --help                       Print help

```

## Examples:
```
rust-cli-archiver --files /home/user/test1.txt --name test_archive.zip
rust-cli-archiver extract --file /home/user/test_archive.rar

```