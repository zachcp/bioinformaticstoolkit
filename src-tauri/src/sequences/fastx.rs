// use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

use std::fs::File;
use std::path::Path;
use std::str;

use serde::{Deserialize, Serialize};

use fasta::record::Definition as FastaDefinition;
use noodles_fasta as fasta;
use noodles_fastq as fastq;

//  Get Stats -------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct FastaData {
    recordcount: i32,
    maxlength: i32,
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_stats(filename: String) -> FastaData {
    let mut count = 0;
    let mut maxlength = 0;

    let mut reader = File::open(&filename)
        .map(BufReader::new)
        .map(fasta::Reader::new)
        .unwrap();

    for result in reader.records() {
        count = count + 1;
        let record = result.unwrap();
        println!("{}\t{}", record.name(), record.sequence().len());
        let reclength = record.sequence().len();

        if reclength > maxlength {
            maxlength = reclength
        }
    }

    let stats = FastaData {
        recordcount: count,
        maxlength: maxlength as i32,
    };

    stats
}

//  Get Advanced Stats  -------------------------------------------------------------------------------------------------------------------

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
    avg_len: f32,
    max_len: i32,
}

#[tauri::command]
pub fn get_seqstats(filename: String) -> SeqKitFastaData {
    let mut count = 0;
    // let mut lengths = vec![] ;
    let mut lengths: Vec<i32> = Vec::new();

    let mut reader = File::open(&filename)
        .map(BufReader::new)
        .map(fasta::Reader::new)
        .unwrap();

    for result in reader.records() {
        count = count + 1;

        let record = result.unwrap();
        println!("{}\t{}", record.name(), record.sequence().len());

        let reclength = record.sequence().len();
        lengths.push(reclength as i32);
    }

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

//  Convert Fastq to Fasta -------------------------------------------------------------------------------------------------------------------

pub fn convert_fastq_to_fasta(input_path: &str, output_path: &str) -> io::Result<()> {
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
        let recname = String::from_utf8(record.name().to_vec()).unwrap();
        let recdescription = String::from_utf8(record.description().to_vec()).unwrap();
        let fasta_definition = FastaDefinition::new(recname, Some(recdescription));
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
pub fn convert_fastq_to_fasta_tauri(input_path: &str, output_path: &str) -> Result<String, String> {
    println!("filpaths: {}  and {} ", input_path, output_path);
    println!("We're in the conversion funtion!");
    let results = convert_fastq_to_fasta(input_path, output_path);
    println!("We've exited the conversion function!");
    if results.is_ok() {
        Ok(format!("Fasta file {output_path} has been created!"))
    } else {
        Err("This failed!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use filesize::PathExt;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_stats() {
        let result = get_stats("testdata/fastx/small.fasta".to_string());
        let FastaData {
            maxlength,
            recordcount,
        } = result;
        assert_eq!(maxlength, 34);
        assert_eq!(recordcount, 2);
    }

    #[test]
    fn test_convert_fastq_fasta() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let output_path = temp_file.path().to_str().unwrap();
        let result = convert_fastq_to_fasta("testdata/fastx/small.fastq", output_path);

        assert!(result.is_ok());
        assert!(temp_file.path().exists());
        assert!(temp_file.path().size_on_disk().unwrap() != 0);
    }

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
