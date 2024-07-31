# The Bioinformatics Toolkit

RUST-backed utilities for bioinformatic data processing. Rust on the backend with JS on the front.

![](src-tauri-icons/Square142x142Logo.png)


## Develop

```sh
# assuming quarto and cargo are installed and on your path.
git clone https://github.com/zachcp/bioinformaticstoolkit.git

cd bioinformaticstoolkit

# install the tauri cli
cargo install tauri-cli

# add cargo bind dir to the path
export PATH=$PATH:~/.cargo/bin/

# to develop
npm run tauri dev

# to package. this build is ~8MB.
npm run tauri build

# to test
cd src-tauri && cargo test
# or verbose
cd src-tauri && cargo test -- --nocapture

```

### Screenshots

![](images/intro.png)
![](images/guide.png)
![](images/rna.png)
![](images/fasta_histogram)
![](images/translation.png)
