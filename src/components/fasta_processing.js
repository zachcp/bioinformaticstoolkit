import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "npm:@tauri-apps/api/core";
import { html } from "npm:htl";
import * as Plot from "npm:@observablehq/plot";

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

export function stats_description(stats) {
  return html`
    <p>AVG Len: ${stats.avg_len}</p>
    <p>Filename: ${stats.filename}</p>
    <p>Format: ${stats.format}</p>
    <p>MAX Len: ${stats.max_len}</p>
    <p>MIN Len: ${stats.min_len}</p>
    <p>Number of Records Len: ${stats.num_seqs}</p>
    <p>Sum Len: ${stats.sum_len}</p>
  `;
}

export function plot_stats(stats) {
  return Plot.rectY(
    { length: 10000 },
    Plot.binX({ y: "count" }, { x: stats.contig_lengths }),
  ).plot();
}

export async function getFastaStatsSeqkit() {
  const fasta = await choosefasta();
  const stats = await invoke("get_seqstats", { filename: fasta });
  return stats;
}
