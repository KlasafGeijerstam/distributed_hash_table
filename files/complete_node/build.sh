#!/bin/bash
cargo build --release
if [ "$(uname)" == "Darwin" ]; then
	cp target/release/tracker target/release/client target/release/node bin/osx/
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
	cp target/release/tracker target/release/client target/release/node bin/linux/
fi
