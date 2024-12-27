cli:
    cargo run --bin weather-cli

build:
    cargo build

test:
    cargo test

fmt:
    rustfmt weather-cli/src/*.rs weather-core/src/*.rs

check:
    rustfmt --check weather-cli/src/*.rs weather-core/src/*.rs

clean:
    cargo clean

update:
    cargo update
