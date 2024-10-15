# Restriction Enzyme Analysis

- Currently will search for 3 specific recognition sites in the input string.
- It would be nie to have a fully importable RE library of REs similar to the one in Biopython.
- Try: `CCCCCGGGCGCGGCGCGCGAAAAAGGGGGGGGGGGGGG`


```js
import { invoke } from "npm:@tauri-apps/api/core";
let text = view(Inputs.textarea({label: "DNA For Searching", placeholder: "DNA Goes here", submit: true}));
// let re_sites =  (text == "") ? []: invoke("check_restriction_sites", {sequence: text})
text
```

```js

let re_sites;
if (text != "") {
    re_sites= await invoke("check_restriction_sites", {sequence: text});
    display(re_sites)
}

// display( re_sites)

```
