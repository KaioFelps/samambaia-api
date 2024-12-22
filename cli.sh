#/bin/bash
samambaia_cli_path='./target/release/samambaia-cli.exe'

if [ ! -e "$samambaia_cli_path" ]; then
    cargo build --release -p samambaia-cli
fi

alias samambaia="$samambaia_cli_path"
