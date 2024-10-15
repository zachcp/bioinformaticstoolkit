# Gosling Visualization


<div id="gosling-container"></div>

<head>
  <link
    rel="stylesheet"
    href="https://esm.sh/higlass@1.13/dist/hglib.css"
  />
</head>

<script type="importmap">
  {
    "imports": {
      "react": "https://esm.sh/react@18",
      "react-dom": "https://esm.sh/react-dom@18",
      "pixi": "https://esm.sh/pixi.js@6",
      "higlass": "https://esm.sh/higlass@1.13?external=react,react-dom,pixi",
      "gosling.js": "https://esm.sh/gosling.js@0.11.0?external=react,react-dom,pixi,higlass"
    }
  }
</script>

```js
import { embed } from 'npm:gosling.js';
    
embed(document.getElementById('gosling-container'), {
    "tracks": [
      {
        "data": {
          "url": "https://server.gosling-lang.org/api/v1/tileset_info/?d=cistrome-multivec",
          "type": "multivec",
          "row": "sample",
          "column": "position",
          "value": "peak",
          "categories": ["sample 1"],
        },
        "mark": "rect",
        "x": { "field": "position", "type": "genomic" },
        "color": { "field": "peak", "type": "quantitative", "legend": true },
        "width": 600,
        "height": 130,
      },
    ],
});

```

