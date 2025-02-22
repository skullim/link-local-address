default:
    just --list

setup-interface interface_name:
    ./scripts/setup_interface.sh  {{ interface_name }}

remove-interface interface_name:
    ./scripts/remove_interface.sh {{ interface_name }}

_assign-capabilities script:
    #!/usr/bin/env sh
    for file in $( .{{ script }}); do
    ./scripts/setup_net_cap.sh "$file"
    done

test:
    cargo test --no-run
    just _assign-capabilities /scripts/find_test_files.sh

    #!/usr/bin/env sh
    for test_name in $(./scripts/list_tests.sh); do \
        just setup-interface dummy0; \
        cargo test "$test_name"; \
        just remove-interface dummy0; \
    done

run-cli interface mac_addr:
    cargo build
    just _assign-capabilities /scripts/find_binary.sh
    cargo run --bin link-local-address-cli -- -i {{ interface }} -m {{ mac_addr }}

publish:
    cargo build --all-targets
    just test
    cargo publish -p link-local-address
    cargo publish -p link-local-address-cli
