// See https://observablehq.com/framework/config for documentation.
export default {
  // The project’s title; used in the sidebar and webpage titles.
  title: "bioinformatics-toolkit",
  search: true,
  // The pages and sections in the sidebar. If you don’t specify this option,
  // all pages will be listed in alphabetical order. Listing pages explicitly
  // lets you organize them into sections and have unlisted pages.
  pages: [
    {
      name: "Fasta Utiliies",
      pages: [
        {name: "Simple", path: "fastx/fasta_simple"},
        {name: "FQ->Fasta", path: "fastx/fastq_to_fasta"},
        {name: "Fastx Stats", path: "fastx/fastx_stats"},
      ]
    },
    {
      name: "DNA Utiliies",
      pages: [
        {name: "RE_Analysis", path: "dna/restriction_enzyme_analysis"},
        {name: "Translation", path: "dna/translation"},
      ]
    },
    {
      name: "RNA Utiliies",
      pages: [
        {name: "RNAnapkin", path: "rna/rna_rnapkin_viz"},
      ]
    },
    {
      name: "Javascript Utiliies",
      pages: [
        {name: "IGVJS", path: "JSApps/igvjs"},
        // {name: "Gosling", path: "JSApps/gosling"},
        {name: "graphgenomeviewer", path: "JSApps/genomegraph"},
        {name: "msaviewer", path: "JSApps/msa-viewer"},
      ]
    },
  ],


  // Some additional configuration options and their defaults:
  theme: "default", // try "light", "dark", "slate", etc.
  // header: "", // what to show in the header (HTML)
  // footer: "Built with Observable.", // what to show in the footer (HTML)
  toc: true, // whether to show the table of contents
  // pager: true, // whether to show previous & next links in the footer
  // root: "docs", // path to the source root for preview
  // output: "dist", // path to the output root for build
};
