
mod sequences;
use sequences::fastx::{convert_fastq_to_fasta_tauri, get_seqstats, get_stats};
use sequences::patterns::check_restriction_sites;
use sequences::rnapkin::rnapkin_fn;
use sequences::dna_utils::translate_dna;


  
#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_stats,
            get_seqstats,
            rnapkin_fn,
            check_restriction_sites,
            convert_fastq_to_fasta_tauri,
            translate_dna
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}