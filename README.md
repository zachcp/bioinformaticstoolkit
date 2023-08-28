# The Bioinformatics Toolkit

RUST-backed utilities for bioinformatic data processing.


## Get started

```sh
# assuming quarto and cargo are installed and on your path.

# install the tauri cli
cargo install tauri-cli

# add cargo bind dir to the path
export PATH=$PATH:~/.cargo/bin/

# to develop 
cargo-tauri dev

# to package. this build is ~8MB. 
cargo-tauri build
```