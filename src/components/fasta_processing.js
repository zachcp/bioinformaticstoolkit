import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "npm:@tauri-apps/api/core";
import { html } from "npm:htl";

function choosefasta() {
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

function choosefastq() {
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

export async function getFastaStats() {
  const fasta = await choosefasta();
  const stats = await invoke("get_stats", { filename: fasta });
  return html`<div>
    <h3>Fasta Statistics</h3>
    <p>Record Count: ${stats.recordcount}</p>
    <p>Maximum Length: ${stats.maxlength}</p>
  </div>`;
}
