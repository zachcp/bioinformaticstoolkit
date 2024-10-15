---
title: "Simple Fasta"
---

Simplest example. uses Noodles to loop throught a fasta file and return the record count and maximul length in the file

```js
import {getFastaStats} from "../components/fasta_processing.js";

const fasta_stats = view(
Inputs.button("Get Fasta Stats", {
  value: null,
  reduce: () => getFastaStats()
}));
```

```js
// if null there is a spinning arrow.
if (fasta_stats != null) {
  display(fasta_stats)
}
```
