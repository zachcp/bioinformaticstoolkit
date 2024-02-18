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

const fasta_stats_seqkit = view(Inputs.button(
    "Get Fasta Stats", 
    { 
        value: null, 
        reduce: () => choosefasta().then((fname) => invoke("get_seqstats", {filename: fname.path})) 
    }));


let fasta_stats_seqkit_realized = (fasta_stats_seqkit == null)  ? "Click Above to Get Fasta Statistics" : fasta_stats_seqkit

```

```js
display(fasta_stats_seqkit_realized)
```




## Basic Stats

```js


// html`AVG Len: ${fasta_stats_seqkit_realized.avg_len}`
// html`Filename: ${fasta_stats_seqkit_realized.filename}`
// html`Format: ${fasta_stats_seqkit_realized.format}`
// html`MAX Len: ${fasta_stats_seqkit_realized.max_len}`
// html`MIN Len: ${fasta_stats_seqkit_realized.min_len}`
// html`Number of Records Len: ${fasta_stats_seqkit_realized.num_seqs}`
// html`Sum Len: ${fasta_stats_seqkit_realized.sum_len}`

```

## Plot Histogram 

```js

// display(
// Plot.rectY(
//             {length: 10000}, 
//             Plot.binX(
//                 {y: "count"}, 
//                 {x: fasta_stats_seqkit_realized.contig_lengths})).plot());

```