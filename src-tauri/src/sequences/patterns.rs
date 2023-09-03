// use std::io::prelude::*;
use bio::pattern_matching::bndm;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
pub struct REMatches {
    name: String,
    locations: Vec<usize>,
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
