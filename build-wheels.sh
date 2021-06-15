#!/bin/bash
set -ex

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
export PATH="$HOME/.cargo/bin:$PATH"


for pyver in "" "3.7" "3.8" "3.9" "3.10"; do
    s=$(which python"$pyver")
    if [ ! -z "$s" ]; then
        pip$pyver install -U setuptools wheel setuptools-rust
        python$pyver setup.py bdist_wheel
    fi
done

