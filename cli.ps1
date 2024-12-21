function hubbitos-cli {
    $global:HubbitosCliPath = "target/release/hubbitos-cli.exe"

    if (-not [System.IO.File]::Exists($HubbitosCliPath)) {
        & cargo build --release -p hubbitos-cli
    }

    & $HubbitosCliPath $args
}