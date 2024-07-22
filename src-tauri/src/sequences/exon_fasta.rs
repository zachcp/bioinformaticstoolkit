use arrow::util::pretty::pretty_format_batches;
use datafusion::error::DataFusionError;
use datafusion::prelude::*;
use exon::ExonSession;

pub fn get_stats(filename: String) -> Result<String> {
    let config = SessionConfig::new()
        .with_target_partitions(4)
        .with_repartition_file_scans(true);

    let ctx = ExonSession::with_config_exon(config);

    let path = "./exon-examples/data/Ga0604745_crt.gff";
    let sql = format!("CREATE EXTERNAL TABLE gff STORED AS GFF LOCATION '{path}';",);
}
