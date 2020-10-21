# Building on Linux/OSX
1. Install [`rustup`](https://rustup.rs/) (Current version build using Rust version 1.47).
2. Run `./build.sh`.
3. Binaries should now be located in the `bin/linux` or `bin/osx` folder.

# Building on Windows
1. Install [`rustup`](https://rustup.rs/) (Current version build using Rust version 1.47).
2. Run `cargo build --release`
3. Copy `tracker.exe`, `client.exe` and `node.exe` from the `target/release/` folder into your preferred location.


# Running binaries
The `bin` directory contains pre-compiled binaries for the tracker, node and test-client.
The binaries are built for linux, if you can not run the files you can compile them
from source.

All binaries have built in help describing the arguments for the program, which can be
shown by running `./{tracker/node/client} --help` (`./tracker.exe` on Windows).

## Tracker

The tracker takes one mandatory argument, the port to listen for incoming UDP traffic.
Optional parameters are shown by running the tracker with the `--help` parameter.

## Node

The `node` binary is an implementation of the described node. Use the `--help` parameter
to see what the parameters are. The node can be shutdown by sending `SIGINT`, either via
terminal or by pressing `CTRL+C` in the terminal. If the node crashes due to a malformed 
PDU or similar, the node can be shut down by sending `SIGQUIT`, via `CTRL+\`.

## Client

The `client` allows you to perform value operations to a node network, as well as bulk inserts
via a `csv` file. A sample `csv` is provided in [data.csv](data.csv). The client can either
connect to a specific node, or use the tracker to get a random node from the network.
Use the `--help` parameter to see the available parameters.
