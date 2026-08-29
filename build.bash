#!/bin/bash

set -euo pipefail

cargo fmt
cargo clippy -- -Wclippy::pedantic -Wclippy::style -Aclippy::missing-panics-doc
cargo build --all-targets

for example_target in example_targets/*; do
    pushd "$example_target"
    make
    popd
done
