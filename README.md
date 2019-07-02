
## Build

```
$ mkdir servo-embedding-example
$ cd servo-embedding-example
$ cargo init --bin --name servo-embedding-example

$ cp ~/src/servo/rust-toolchain .
$ rustup install `cat rust-toolchain`
$ cp ~/src/servo/Cargo.lock .
$ cp -r ~/src/servo/resources . 
$ cargo build --release
$ cargo run --release
```

## Retry after git pull origin master

```
$ rm -rf rust-toolchain
$ rm -rf Cargo.lock
$ rm -rf resources
$ cp ~/src/servo/rust-toolchain .
$ rustup install `cat rust-toolchain`
$ cp ~/src/servo/Cargo.lock .
$ cp -r ~/src/servo/resources . 
$ cargo build --features azure_backend --release
$ cargo run --features azure_backend --release
```

