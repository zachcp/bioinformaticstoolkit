# RNApkin

```js
import {svg, html} from "npm:htl";
import { invoke } from "npm:@tauri-apps/api/core";


function unsafe_html() {
  const span = document.createElement("span");
  span.innerHTML = String.raw.apply(this, arguments);
  return span;
}
```

Utility that uses [rnapkin](https://github.com/ukmrs/rnapkin),  a rust library that shows RNA secondary structure. Try:

```txt

>fantastic guanine riboswitch
AAUAUAAUAGGAACACUCAUAUAAUCGCGUGGAUAUGGCACGCAAGUUUCUACCGGGCAC
..........(..(.((((.((((..(((((.......)))))..........((((((.
CGUAAAUGUCCGACUAUGGGUGAGCAAUGGAACCGCACGUGUACGGUUUUUUGUGAUAUC
......)))))).....((((((((((((((((((........))))))...........
AGCAUUGCUUGCUCUUUAUUUGAGCGGGCAAUGCUUUUUUUA
..)))))))))))).)))).)))).)..).............

> offsam
0000022222222223333333333333333333333333333333333444444444444444444444444444444
AUAUCCGUUCUUAUCAAGAGAAGCAGAGGGACUGGCCCGACGAUGCUUCAGCAACCAGUGUAAUGGCGAUCAGCCAUGA
.......((((((((....(((((...(((.....)))......)))))(((..(((((...(((((.....))))).)
4444444444555555555555555555555555555522222222222211111111111111111111111111111
CUAAGGUGCUAAAUCCAGCAAGCUCGAACAGCUUGGAAGAUAAGAAGAGACAAAAUCACUGACAAAGUCUUCUUCUUAA
))..)).)))........((((((.....))))))...)))))))).................((((((.((((...))
111111111111
GAGGACUUUUUU
)).))))))...
```


```js

const rnapkin_params = view(Inputs.form({
  height: Inputs.range([0, 900], {step: 10, label: "Height", default: 400}),
  angle: Inputs.range([0, 360], {step: 1, label: "Angle", default: 0}),
  bubble_radius: Inputs.range([0, 1], {step: 0.1, label: "Bubble Radius", default: 0.5, visible:false}),
  mirror_x: Inputs.toggle({label: "Flip X", value: false}),
  mirror_y: Inputs.toggle({label: "Flip Y", value: false}),
  color_theme : Inputs.select(["dark", "white", "black", "bright", "default"], {value: "default", label: "Color Theme"}),
  rna: Inputs.textarea({label: "RNA For Searching", placeholder: "RNA Napkin" //, submit: true
  })
}));

```


```js
let rna_text_realized;
let rna_text_realized2;

if (rnapkin_params.rna != "") {
  // retunrs text. I need a propr DOM element
  rna_text_realized = await invoke("rnapkin_fn", {
  sequence: rnapkin_params.rna,
  height: rnapkin_params.height,
  color_theme:  rnapkin_params.color_theme,
   mirror_x: rnapkin_params.mirror_x,
   mirror_y: rnapkin_params.mirror_y,
   rotation_angle:rnapkin_params.angle,
   bubble_radius: rnapkin_params.bubble_radius});
  console.log(rna_text_realized);
  display(unsafe_html`${rna_text_realized}`);
}
