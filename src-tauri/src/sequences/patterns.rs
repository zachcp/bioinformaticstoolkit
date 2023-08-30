// use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

use bio::pattern_matching::bndm;
use noodles_fasta as fasta;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
pub struct REMatches {
    name: String,
    locations: Vec<usize>,
}

fn quick_test() -> Vec<REMatches> {
    // let eco_ri = "GAATTC";
    // let bamhi = "GGATCC";
    // let pattern = b"GAAAA";
    // let text = "ACGGCTGGATCCAGAATTCGAAAAGGCTAGAAAA";
    // let bndm = bndm::BNDM::new(pattern);
    // let occ: Vec<usize> = bndm.find_all(text.as_bytes()).collect();
    // assert_eq!(occ, [7, 17]);
    // println!("{:?}", occ);

    let text = b"ACGGCTGGATCCAGAATTCGAAAAGGCTAGAAAAGGATCC";
    let eco_ri = b"GAATTC";
    let bamhi = b"GGATCC";
    let gcrich = b"GGGGGG";
    let patterns = vec![eco_ri, bamhi, gcrich];

    let mut results: Vec<REMatches> = vec![];

    for patt in patterns {
        let newpatt = bndm::BNDM::new(patt);
        let newout: Vec<usize> = newpatt.find_all(text).collect();

        results.push(REMatches {
            name: str::from_utf8(patt).unwrap().to_string(),
            locations: newout,
        });
    }
    results
}

#[tauri::command]
pub fn check_restriction_sites(sequence: &str) -> Vec<REMatches> {
    let text = sequence.as_bytes();
    let eco_ri = b"GAATTC";
    let bamhi = b"GGATCC";
    let gcrich = b"GGGGGG";

    let mut results: Vec<REMatches> = vec![];
    let patterns = vec![eco_ri, bamhi, gcrich];

    for patt in patterns {
        let newpatt = bndm::BNDM::new(patt);
        let newout: Vec<usize> = newpatt.find_all(text).collect();

        results.push(REMatches {
            name: str::from_utf8(patt).unwrap().to_string(),
            locations: newout,
        });
    }
    results
}
