#/bin/bash
hubbitos_cli_path='./target/release/hubbitos-cli.exe'

if [ ! -e "$hubbitos_cli_path" ]; then
    cargo build --release -p hubbitos-cli
fi

alias hubbitos-cli="$hubbitos_cli_path"
