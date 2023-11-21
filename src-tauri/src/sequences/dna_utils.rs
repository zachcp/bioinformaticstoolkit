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


}
