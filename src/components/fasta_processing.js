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

export function getFastaStats() {
  try {
    // File selection
    const selected = open({
      multiple: false,
      filters: [
        {
          name: "Fasta",
          extensions: ["fa", "fasta", "fna"],
        },
      ],
    });

    if (!selected) {
      return html`<div>No file selected</div>`;
    }

    console.log("Selected file:", selected);

    // Invoke Rust command
    const stats = invoke("get_stats", { filename: selected });

    // Render stats
    return html`
      <div>
        <h3>Fasta Statistics</h3>
        <p>Filename: ${stats.filename}</p>
        <p>Number of Sequences: ${stats.num_seqs}</p>
        <p>Total Length: ${stats.total_length}</p>
        <p>Minimum Length: ${stats.min_length}</p>
        <p>Maximum Length: ${stats.max_length}</p>
        <p>Average Length: ${stats.avg_length.toFixed(2)}</p>
        <p>N50: ${stats.n50}</p>
        <p>GC Content: ${(stats.gc_content * 100).toFixed(2)}%</p>
      </div>
    `;
  } catch (error) {
    console.error("Error:", error);
    return html`<div>Error: ${error.message}</div>`;
  }
}
