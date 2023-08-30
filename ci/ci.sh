#!/bin/bash

echo "Preparing Environment"

sudo apt-get update && sudo apt-get -y install pkg-config \
			       libssl-dev \
			       build-essential \
			       openssl

sudo mkdir /release

# For building a static binary.
export OPENSSL_STATIC=1
export CRATE_NAME=ghost
export CI_TAG=$(cargo pkgid | cut -d "#" -f2)

echo "Building binary"

cargo test
cargo build --release

echo "Pushing to release folder"

sudo cp target/release/ghost /release

echo "Spitting out to github release"
echo "Crate Name: ${CRATE_NAME}"
echo "Tag Info: ${CI_TAG}"

# Tag information available

cd /release

sudo tar -cvf ${CRATE_NAME}-${CI_TAG}.tar.gz *