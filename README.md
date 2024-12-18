# Kernel-Node

An experimental bitcoin node written in Rust using the libbitcoinkernel
library. The node just validates blocks, and does not serve blocks to the
network. It is meant to showcase the limited initial API of the kernel library.
It is not meant to be particularly performant, or robust against misbehaving
peers.

For now, IBD is done from a single peer, selected from the DNS seed nodes. A
direct connection can also be selected from the command line. See `--help` for
this. If the connection fails, the users has to restart and retry manually for
now.

To run on e.g. signet:

```
cargo run --bin kernel-node -- --network signet
```

By default it will put data in the `$HOME/kernel-node` directory.
