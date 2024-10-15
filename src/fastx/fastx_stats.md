---
title: "Fastx stats"
---

```txt
https://downloads.pacbcloud.com/public/dataset/Sequel-IIe-202104/metagenomics/?utm_source=Website&utm_medium=webpage&utm_term=SQII-humanGut-microbiome-pooledStandards&utm_content=datasets&utm_campaign=0000-Website-Leads
https://downloads.pacbcloud.com/public/dataset/AAV/2022-ssAAV-scAAV-mix/1-mapped-mixed/

simple statistics of FASTA/Q files

Columns:

1.  file      input file, "-" for STDIN
2.  format    FASTA or FASTQ
3.  type      DNA, RNA, Protein or Unlimit
4.  num_seqs  number of sequences
5.  sum_len   number of bases or residues       , with gaps or spaces counted
6.  min_len   minimal sequence length           , with gaps or spaces counted
7.  avg_len   average sequence length           , with gaps or spaces counted
8.  max_len   miximal sequence length           , with gaps or spaces counted
9.  Q1        first quartile of sequence length , with gaps or spaces counted
10. Q2        median of sequence length         , with gaps or spaces counted
11. Q3        third quartile of sequence length , with gaps or spaces counted
12. sum_gap   number of gaps
13. N50       N50. https://en.wikipedia.org/wiki/N50,_L50,_and_related_statistics#N50
14. Q20(%)    percentage of bases with the quality score greater than 20
15. Q30(%)    percentage of bases with the quality score greater than 30
16. AvgQual   average quality
17. GC(%)     percentage of GC content

```

## Basic Stats

```js
import {stats_description, plot_stats, getFastaStatsSeqkit} from "../components/fasta_processing.js";

const fasta_stats = view(
Inputs.button("Get Fasta Stats", {
  value: null,
  reduce: () => getFastaStatsSeqkit()
}));

```

```js
if (fasta_stats != null) {
  display(stats_description(fasta_stats));
  display(plot_stats(fasta_stats));
}
```
