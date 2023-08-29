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

# to test
cd src-tauri && cargo test
# or verbose
cd src-tauri && cargo test -- --nocapture

```


## Ideas


FASTX:
  - covert fasta to fastq
  - basic stats of fasta/or fastq 
  - merge PE reads
  - split interleaved
  - advanced stats including lenght histrogram.
      - set some reasonable number of sequences for inclusion in the histrogrma before binning
  - splitting into multiple files.
  - filter bases on quality
  - sample the reads based on a seed.
  - length by quality metrics:
      - optional hexagon plots
  - coverage plots and 
  - digestion locations
  - Genome Card.
    -  Genome name, overview, produces compounds

DNA Analysis:

  - Digestability of DNA sequences:
      - Standard RE sites 
      - Other Patterns to Avoid
      - Genome View 
      - Global view of Palettes and coding types
  - Insilico PCR: https://github.com/dlesl/pcr
    - CLonifier: https://github.com/dlesl/clonifier
  - Phenogram
  - GFATK: https://github.com/tolkit/gfatk
  - Plasmapr: https://github.com/BradyAJohnston/plasmapR
  - Pangenome TK: https://github.com/GeneDx/pgr-tk (cdep in the build)
  - RE digest and assembly calculations
  - Utilities for Codons
  - [VCF plotein](https://vcfplotein.liigh.unam.mx/)
  - [ASGArt](https://github.com/delehef/asgart) (cdep in the build)
  - [UDON](https://github.com/ocxtal/udon)
  - [GFAESTUS](https://github.com/chfi/gfaestus) (c++ dep )
  - [BioSeq](https://github.com/jeff-k/bio-seq)
  - [10x Genomics Rust](https://github.com/10XGenomics/rust-toolbox)
  - [fq pareser](https://crates.io/crates/fastq)
  - [fastats](https://crates.io/crates/fakit)
  - [fqmerge](https://crates.io/crates/fqkit)
  - [ggcat](https://github.com/algbio/ggcat)
  - [light motif](https://crates.io/crates/lightmotif)
  - [liftover with crusmapr](liftover)

  - [exon](https://docs.rs/exon/latest/exon)
  - [phylogeny](https://docs.rs/phylogeny/latest/phylogeny/) # not much action

  - [chemical Reaction networks](https://lib.rs/crates/rebop)
  - [gb-io](https://lib.rs/crates/gb-io)
  - [charming - a nive gui library](https://github.com/yuankunzhang/charming)
  - [met map](https://lib.rs/crates/shu)

 - [barcode counter](https://lib.rs/crates/barcode-count)
 - [hpo](https://lib.rs/crates/hpo)
 - nanopore read assessment: https://lib.rs/crates/nanoq#readme-read-report
 - [niffler](https://github.com/luizirber/niffler/)
 - [OBO Validatio](https://lib.rs/crates/fastobo-validator)
 - [rustyms](https://lib.rs/crates/rustyms)
 - [preotienogenic](https://lib.rs/crates/proteinogenic)
 - [rdkit](https://lib.rs/crates/rdk)
 - [bigwig2bam](https://lib.rs/crates/bigwig2bam)



VCF:
    - convert
    - concat
    - split


rna-seq:
    - gencounts https://github.com/NKI-GCF/gensum
    - rust-lapper https://crates.io/crates/rust-lapper


Taxonomy:
    - load and display a tree file
    - load and display kraken
    - load and display bracken

Peptides and Proteomics: 
  - [unipept](https://crates.io/crates/umgap)

  


Software: 

- [Pangenomer](https://github.com/marschall-lab/panacus)
- [rust-bio](https://github.com/rust-bio/rust-bio)
- [rust-bio-tools](https://github.com/rust-bio/rust-bio-tools)

