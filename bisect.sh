#!/bin/sh

set -ex

# This checks that a warning is only printed once.
# See https://github.com/rust-lang/rust/issues/88256 for a regression where it
# started printing twice.

cargo test --doc
