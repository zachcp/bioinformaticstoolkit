import { dialog, invoke } from "@tauri-apps/api";

let open = dialog.open;

async function choosefasta() {
  const selected = open({
    multiple: false,
    filters: [
      {
        name: "Fasta",
        extensions: ["fa", "fasta", "fna"],
      },
    ],
  });
  return selected;
}

async function choosefastq() {
  const selected = open({
    multiple: false,
    filters: [
      {
        name: "Fastq",
        extensions: ["fq", "fastq"],
      },
    ],
  });
  return selected;
}

const fasta_stats = view(
  Inputs.button("Get Fasta Stats", {
    value: null,
    reduce: () =>
      choosefasta().then((fname) => {
        console.log("Selected file:", fname);
        return invoke("get_stats", { filename: fname });
      }),
  }),
);

let fasta_stats_realized =
  fasta_stats == null ? "Click Above to Get Fasta Statistics" : fasta_stats;
