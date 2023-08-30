// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;
use std::str;
use noodles_fasta as fasta;
use serde::{Deserialize, Serialize};
use bio::pattern_matching::bndm;

mod sequences;
use sequences::fastx as fastx;
use sequences::fastx::convert_fastq_to_fasta_tauri;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastaData {
    recordcount: i32,
    maxlength: i32,
}


#[tauri::command]
fn get_stats(filename: String) -> FastaData {

    let mut count = 0;
    let mut maxlength = 0;

    let mut reader = File::open(&filename)
      .map(BufReader::new)
        .map(fasta::Reader::new).unwrap();

    for result in reader.records() {
        count = count + 1;
        let record = result.unwrap();
        println!("{}\t{}", record.name(), record.sequence().len());
        let reclength =  record.sequence().len();

        if reclength > maxlength {
            maxlength = reclength
        }
    };

    let stats = FastaData {
        recordcount: count,
        maxlength: maxlength as i32,
    };

    stats
}



// simple statistics of FASTA/Q files
// Columns:
//   1.  file      input file, "-" for STDIN
//   2.  format    FASTA or FASTQ
//   3.  type      DNA, RNA, Protein or Unlimit
//   4.  num_seqs  number of sequences
//   5.  sum_len   number of bases or residues       , with gaps or spaces counted
//   6.  min_len   minimal sequence length           , with gaps or spaces counted
//   7.  avg_len   average sequence length           , with gaps or spaces counted
//   8.  max_len   miximal sequence length           , with gaps or spaces counted
//   9.  Q1        first quartile of sequence length , with gaps or spaces counted
//   10. Q2        median of sequence length         , with gaps or spaces counted
//   11. Q3        third quartile of sequence length , with gaps or spaces counted
//   12. sum_gap   number of gaps
//   13. N50       N50. https://en.wikipedia.org/wiki/N50,_L50,_and_related_statistics#N50
//   14. Q20(%)    percentage of bases with the quality score greater than 20
//   15. Q30(%)    percentage of bases with the quality score greater than 30
//   16. GC(%)     percentage of GC content
//
// Basic is sum_len min_len avg_len max_len
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeqKitFastaData {
    filename: String,
    format: String,
    datatype: String,
    num_seqs: i32,
    sum_len: i32,
    min_len: i32,
    avg_len:f32,
    max_len: i32,
}



#[tauri::command]
fn get_seqstats(filename: String) -> SeqKitFastaData {

    let mut count = 0;
    // let mut lengths = vec![] ;
    let mut lengths: Vec<i32> = Vec::new();


    let mut reader = File::open(&filename)
      .map(BufReader::new)
        .map(fasta::Reader::new).unwrap();

    for result in reader.records() {
        count = count + 1;

        let record = result.unwrap();
        println!("{}\t{}", record.name(), record.sequence().len());
        
        let reclength =  record.sequence().len();
        lengths.push(reclength as i32);
    };

    
    let total = lengths.iter().sum();
    let min_value = *lengths.iter().min().unwrap_or(&0);    
    let max_value = *lengths.iter().max().unwrap_or(&0);
    
    let avg = (total / count) as f32;


    let stats = SeqKitFastaData {
        filename: filename,
        format: "Fasta".to_string(),
        datatype: "DNA".to_string(),
        num_seqs: count,
        sum_len: total,
        avg_len: avg,
        min_len: min_value,
        max_len: max_value,
    };

    stats
}





#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct REMatches {
    name: String,
    locations: Vec<usize>
}

fn quick_test() -> Vec<REMatches>{
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
        
        results.push(
            REMatches {
                name: str::from_utf8(patt).unwrap().to_string(),
                locations: newout,
            }
        );     
    }
    results
}



#[tauri::command]
fn check_restriction_sites(sequence: &str) -> Vec<REMatches> {

    let text = sequence.as_bytes();
    let eco_ri = b"GAATTC";
    let bamhi = b"GGATCC";
    let gcrich = b"GGGGGG";

    let mut results: Vec<REMatches> = vec![];
    let patterns = vec![eco_ri, bamhi, gcrich];
    

    for patt in patterns {
        let newpatt = bndm::BNDM::new(patt);
        let newout: Vec<usize> = newpatt.find_all(text).collect();
        
        results.push(
            REMatches {
                name: str::from_utf8(patt).unwrap().to_string(),
                locations: newout,
            }
        );      
    }
    results
    
}





fn main() {


    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![
                greet, get_stats, get_seqstats, check_restriction_sites, 
                convert_fastq_to_fasta_tauri])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_small_stats() {
        let result = get_stats("testdata/fastx/small.fasta".to_string());
        let FastaData {maxlength, recordcount} = result;
        assert_eq!(maxlength, 34);
        assert_eq!(recordcount, 2);
    }



}
