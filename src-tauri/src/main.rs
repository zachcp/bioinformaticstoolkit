// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

mod sequences;
use sequences::fastx::{self, convert_fastq_to_fasta_tauri, get_seqstats, get_stats};
use sequences::patterns::check_restriction_sites;
use sequences::rnapkin::rnapkin_fn;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_stats,
            get_seqstats,
            rnapkin_fn,
            check_restriction_sites,
            convert_fastq_to_fasta_tauri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
}
