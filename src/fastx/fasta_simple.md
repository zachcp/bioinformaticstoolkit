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


```js
import {getFastaStats} from "../components/fasta_processing.js";
```

```js
const fasta_stats2 = view(
Inputs.button("Get Fasta Stats: Part 2 ", {
  value: null,
  reduce: () => getFastaStats()
}));
```

```js
import {timeline} from "../components/timeline.js";
```

```js
let events = [
  {"name": "Sputnik 1", "year": 1957, "y": 10},
  {"name": "Apollo 11", "year": 1969, "y": 20},
  {"name": "Viking 1 and 2", "year": 1975, "y": 30},
  {"name": "Space Shuttle Columbia", "year": 1981, "y": 40},
  {"name": "Hubble Space Telescope", "year": 1990, "y": 50},
  {"name": "ISS Construction", "year": 1998, "y": 60}
];

display(timeline(events, {height: 300}));

```
