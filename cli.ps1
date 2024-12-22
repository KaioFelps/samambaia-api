function samambaia {
    $global:SamambaiaCliPath = "target/release/samambaia-cli.exe"

    if (-not [System.IO.File]::Exists($SamambaiaCliPath)) {
        & cargo build --release -p samambaia-cli
    }

    & $SamambaiaCliPath $args
}