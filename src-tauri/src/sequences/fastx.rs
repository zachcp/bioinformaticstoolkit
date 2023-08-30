// use std::io::prelude::*;
use std::io::{
    self,
    BufReader,
    BufWriter,
};

use std::fs::File;
use std::path::Path;
use std::str;

use fasta::record::Definition as FastaDefinition;
use noodles_fasta as fasta;
use noodles_fastq as fastq;



pub fn convert_fastq_to_fasta(input_path: &str, output_path: &str)  ->  io::Result<()> {

    
    let mut reader = File::open(input_path)
        .map(BufReader::new)
        .map(fastq::Reader::new)?;

    // note we are creating the file here instead of opening it
    let mut fasta_writer = File::create(output_path)
        .map(BufWriter::new)
        .map(fasta::Writer::new)?;

    for result in reader.records() {
        
        // this is all to convert from Fastq to fasta. Bit of a pain but ....
        let record = result?;
        let recname =  String::from_utf8(record.name().to_vec()).unwrap();
        let recdescription =  String::from_utf8(record.description().to_vec()).unwrap();
        let fasta_definition = FastaDefinition::new(
            recname,
            Some(recdescription),
        );
        let fasta_record = fasta::Record::new(
            fasta_definition,
            fasta::record::Sequence::from(record.sequence().to_vec()),
        );
        
        fasta_writer.write_record(&fasta_record)?;
    }

    Ok(())

}




// Note: in theory I shouldn't need this function at all but.... here it is.
// the issue is that the ```
#[tauri::command(rename_all = "snake_case")]
pub fn convert_fastq_to_fasta_tauri(input_path: &str, output_path: &str)  -> Result<String, String> {
    println!("filpaths: {}  and {} ", input_path, output_path);
    println!("We're in the conversion funtion!");
    let results = convert_fastq_to_fasta( input_path, output_path);
    println!("We've exited the conversion function!");
    if results.is_ok() {
        Ok( format!("Fasta file {output_path} has been created!")) 
    } else {
        Err("This failed!".to_string())
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use filesize::PathExt;


    #[test]
    fn test_convert_fastq_fasta() {
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let output_path = temp_file.path().to_str().unwrap();
        let result = convert_fastq_to_fasta(
            "testdata/fastx/small.fastq",
            output_path,
        );
        
        assert!(result.is_ok());
        assert!(temp_file.path().exists());
        assert!(temp_file.path().size_on_disk().unwrap() != 0);

    }


    #[test]
    fn test_convert_fastq_fasta_tauri() {
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let output_path = temp_file.path().to_str().unwrap();
        let result = convert_fastq_to_fasta_tauri(
            "testdata/fastx/small.fastq",
            output_path,
        );

        assert!(result.is_ok());
        assert!(temp_file.path().exists());
        assert!(temp_file.path().size_on_disk().unwrap() != 0);

    }


}