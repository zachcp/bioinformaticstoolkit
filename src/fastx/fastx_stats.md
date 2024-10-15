---
title: "Fastx stats"
---


```js
let open = window.__TAURI__.dialog.open;
let invoke = window.__TAURI__.core.invoke;

// use the tauri open to get absolute file paths
async function choosefasta(){
    const selected = open({
        multiple: false,
        filters: [{
            name: 'Fasta',
            extensions: ['fa', 'fasta', 'fna']}]
    });
    return selected
};

async function choosefastq(){
    const selected = open({
        multiple: false,
        filters: [{
            name: 'Fastq',
            extensions: ['fq', 'fastq']}]
    });
    return selected
};

```


```js

// const fasta_stats_seqkit = view(Inputs.button(
//    "Get Fasta Stats",
//    {
//        value: null,
//        reduce: () => choosefasta().then((fname) => invoke("get_seqstats", {filename: fname.path}))
//    }));

const fasta_stats_seqkit = view(Inputs.button(
    "Get Fasta Stats",
    {
        value: null,
        reduce: () => choosefasta().then((selected) => {
            if (selected) {
                console.log("selected!");
                return invoke("get_seqstats", { filename: selected }).then(result => {
                                        console.log("Stats received:", result);
                                        return result; // This will update the button's value
                                    });
            } else {
                return Promise.reject("No file selected");
            }
        })
    }
));

let fasta_stats_seqkit_realized = (fasta_stats_seqkit == null)  ? "Click Above to Get Fasta Statistics" : fasta_stats_seqkit

```


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

if (fasta_stats_seqkit_realized !== null) {
    html`AVG Len: ${fasta_stats_seqkit_realized.avg_len}`
    html`Filename: ${fasta_stats_seqkit_realized.filename}`
    html`Format: ${fasta_stats_seqkit_realized.format}`
    html`MAX Len: ${fasta_stats_seqkit_realized.max_len}`
    html`MIN Len: ${fasta_stats_seqkit_realized.min_len}`
    html`Number of Records Len: ${fasta_stats_seqkit_realized.num_seqs}`
    html`Sum Len: ${fasta_stats_seqkit_realized.sum_len}`
}

```


## Plot Histogram

```js
if (fasta_stats_seqkit_realized !== null) {
    display(fasta_stats_seqkit_realized)
}
```


```js
if (fasta_stats_seqkit_realized !== null) {
    display(
        Plot.rectY(
            {length: 10000},
            Plot.binX(
                {y: "count"},
                {x: fasta_stats_seqkit_realized.contig_lengths})).plot());
}
```
