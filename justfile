default:
    just --list

create-dummy-interface:
    ./scripts/create_dummy_interface.sh

remove-dummy-interface:
    ./scripts/remove_dummy_interface.sh

assign-capabilities script:
    #!/usr/bin/env sh
    for file in $( .{{ script }}); do
    ./scripts/setup_net_cap.sh "$file"
    done

test:
    cargo test --no-run
    just assign-capabilities /scripts/find_test_files.sh

    just create-dummy-interface
    cargo test
    just remove-dummy-interface

run:
    cargo build
    just assign-capabilities /scripts/find_binary.sh
    just create-dummy-interface
    cargo run
    just remove-dummy-interface

publish:
    cargo build --all-targets
    just test
    cargo publish
