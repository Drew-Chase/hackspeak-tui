set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]
set shell := ["bash", "-c"]

dev:
    cargo run --profile dev --package hackspeak

clean:
    cargo clean

build:
    cargo build --release