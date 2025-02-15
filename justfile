default:
    just --list

init-network:
    ./scripts/setup_dummy_interface.sh

assign-capabilities script:
    #!/usr/bin/env sh
    for file in $( .{{ script }}); do
    ./scripts/setup_net_raw.sh "$file"
    done

build-tests:
    cargo test --no-run

run-tests:
    cargo test

publish:
    cargo build --all-targets
    just test
    cargo publish

run:
    cargo build
    just assign-capabilities /scripts/find_binary.sh
    cargo run
