// use std::io::prelude::*;
use protein_translate::translate;


//  DNA Utilities -------------------------------------------------------------------------------------------------------------------

#[tauri::command]
pub fn translate_dna(dnainput: String) -> String {
    translate(dnainput.as_bytes())
}

// https://github.com/SecureDNA/quickdna/blob/main/src/rust_api.rs#L410

#[cfg(test)]
mod tests {
    use super::*;
    use filesize::PathExt;
    use tempfile::NamedTempFile;

    #[test]
    fn test_convert_fastq_fasta_tauri() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let output_path = temp_file.path().to_str().unwrap();
        let result = convert_fastq_to_fasta_tauri("testdata/fastx/small.fastq", output_path);

        assert!(result.is_ok());
        assert!(temp_file.path().exists());
        assert!(temp_file.path().size_on_disk().unwrap() != 0);
    }
}
