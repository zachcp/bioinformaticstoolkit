---
title: "Simple Fasta"
---

Simplest example. uses Noodles to loop throught a fasta file and return the record count and maximul length in the file

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

const fasta_stats = view(
  Inputs.button("Get Fasta Stats", {
    value: null,
    reduce: () => choosefasta().then((fname) => {
      console.log('Selected file:', fname);
      return invoke("get_stats", { filename: fname });
    })
  }),
);

let fasta_stats_realized =
  fasta_stats == null ? "Click Above to Get Fasta Statistics" : fasta_stats;

```


```js
display(fasta_stats_realized)
```
