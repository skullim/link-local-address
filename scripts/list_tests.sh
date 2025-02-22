#!/bin/bash
cargo test -- --list | awk '/^[a-zA-Z0-9_:]+::[a-zA-Z0-9_]+:/ {print $1}' | sed 's/\:$//'
