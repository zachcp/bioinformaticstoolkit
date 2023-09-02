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
  - [x] convert fasta to fastq
  - [x] basic stats of fasta/fastq 
  - [ ] histrogram of read lengths (possibly set max number)
  - [ ] merge PE reads // split interleaved
  - [ ] splitting into multiple files ( create directory ?)
  - [ ] filter-fastx length // quality
  - [ ] sample the fasta/x files
  - [ ] plot: length x quality metrics ( optional hexagon plots ) 
  - [ ] plot: coverage by location. 

GFA:
  - [ ] Utilites from [GFATK](https://docs.rs/gfatk/latest/gfatk/) including filtering
    - [ ] GFAStats


DNA Analysis:
  - Digestability of DNA sequences:
      - [ ] Search for RE locations
      - [ ] Other Patterns to Avoid
      - [ ] Data: Standard RE enzymes
      - [ ] Plot: Genome View of RE sites.
      - [ ] Global view of Palettes and coding types
  - Insilico PCR: https://github.com/dlesl/pcr
    - Clonifier: https://github.com/dlesl/clonifier

  - Phenogram
  - Pangenome TK: https://github.com/GeneDx/pgr-tk (cdep in the build)
  - RE digest and assembly calculations

Miscelleaneous:
  - Genome Card: e.g viz with global genome statistics.
    - Genome name, overview, produces compounds
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
 - Plasmapr: https://github.com/BradyAJohnston/plasmapR
 - [flate2](https://docs.rs/flate2/latest/flate2/)use flate2::read::MultiGzDecoder;
 - [bio_streams](https://github.com/jeff-k/bio-streams) 
  - Streaming iterators for bioinformatics data 


VCF:
    - convert
    - concat
    - split

RNA Secondary Structure:
  - RNApkin https://lib.rs/crates/rnapkin


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

