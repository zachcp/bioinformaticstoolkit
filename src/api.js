// Define all your JS-->Rust functions here


// this is required
const { invoke } = window.__TAURI__.tauri;

// Define all of your imports here

export function greet(name_str) {
  return invoke("greet", { name: name_str });
}

