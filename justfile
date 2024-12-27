cli:
    cargo run --bin weather-cli

build verbose='':
    cargo build {{verbose}}

test verbose='':
    cargo test {{verbose}}

fmt:
    rustfmt weather-cli/src/*.rs weather-core/src/*.rs

check verbose='':
    rustfmt --check weather-cli/src/*.rs weather-core/src/*.rs {{verbose}}

clean:
    cargo clean

update:
    cargo update
