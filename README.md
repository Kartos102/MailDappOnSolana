### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build-bpf --manifest-path=./src/program-rust/Cargo.toml --bpf-out-dir=dist/program
$ solana program deploy dist/program/mail_dapp.so
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```
 ### deploy address

 https://solscan.io/account/EDKvsBWEB9aG9eA5g53oQ2r7yK7jsfzAgyjMgozhDKmZ?cluster=devnet