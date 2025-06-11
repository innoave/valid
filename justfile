#!/usr/bin/env just --justfile

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build
alias c := check
alias cc := code-coverage
alias d := doc
alias l := lint
alias la := lint-all-features
alias ld := lint-default
alias t := test
alias ta := test-all-features
alias td := test-default

# list recipies
default:
    just --list

# build the crate for debugging
build:
    cargo build --all-features

# check syntax in all targets
check:
    cargo check --all-targets --all-features

# linting code using Clippy
lint:
    just lint-all-features
    just lint-default
# currently per default no features are enabled
#    just lint-no-features

# linting code using Clippy with all features enabled
lint-all-features:
    cargo clippy --all-targets --all-features

# linting code using Clippy with default features enabled
lint-default:
    cargo clippy --all-targets

# linting code using Clippy with no features enabled
lint-no-features:
    cargo clippy --all-targets --no-default-features

# run all tests
test:
    just test-all-features
    just test-default
# currently per default no features are enabled
#    just test-no-features

# run tests for all features
test-all-features:
    cargo test --all-features

# run tests for default features
test-default:
    cargo test

# run tests with no features enabled
test-no-features:
    cargo test --no-default-features

# run code coverage (does not include doc-tests)
code-coverage:
    cargo +nightly llvm-cov clean
    cargo +nightly llvm-cov --branch --all-features --no-report
    cargo +nightly llvm-cov report --html --open --ignore-filename-regex "tests|test_dsl"

# build the crate for release
build-release:
    cargo build --release

# clean the workspace
clean:
    cargo clean

# generate and open docs locally
doc $RUSTDOCFLAGS="--cfg docsrs":
    cargo +nightly doc --all-features --no-deps --open
