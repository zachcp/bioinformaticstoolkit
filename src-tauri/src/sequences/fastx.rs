use tempfile::NamedTempFile;

// use std::io::prelude::*;
use std::io::{
    self,
    BufReader,
    BufWriter,
};

use std::str;
use std::fs::File;

use fasta::record::Definition as FastaDefinition;
use noodles_fasta as fasta;
use noodles_fastq as fastq;




pub fn fastq_to_fasta(input_path: &str, output_path: &str)  -> io::Result<()>  {

    let mut reader = File::open(input_path)
        .map(BufReader::new)
        .map(fastq::Reader::new)?;

    let mut fasta_writer = File::open(output_path)
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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_fastq_fasta() {
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let output_path = temp_file.path().to_str().unwrap();
        let result = fastq_to_fasta(
            "testdata/fastx/small.fastq",
            output_path,
        );

        assert!(result.is_ok());

    }


}