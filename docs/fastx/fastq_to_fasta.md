---
title: "Fastq to fasta conversion"
---

This function will take a fasq file and convert it to a fasta file.


```js
let open = window.__TAURI__.dialog.open;
let invoke = window.__TAURI__.core.invoke;
```


```js

async function choosefastq(){
    const selected = open({
        multiple: false,
        filters: [{
            name: 'Fastq',
            extensions: ['fq', 'fastq']}]
    });
    return selected
};

const convert_stats = view(Inputs.button(
    "Convert FastQ to Fasta", 
    { 
        value: null, 
        reduce: () => choosefastq()
            .then( function (fqname) {
                // console.log(fqname);
                let fasta_name = fqname.path.split(".").slice(0,-1).join(".");
                fasta_name = fasta_name + "_converted.fasta"
                return invoke("convert_fastq_to_fasta_tauri", {input_path: fqname.path, output_path: fasta_name})})}));

let convert_stats_realized = (convert_stats == null) ? "Click Above to Convert" : convert_stats

```


```js
view(convert_stats_realized)
```
