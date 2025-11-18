# Nanonis Utilities (Rust Workspace)

This repository contains a Rust **workspace** for processing and visualizing
Nanonis SPM (`.dat`) data.  
It includes two crates:

- **nanonis-parse** — parse Nanonis SPM spectrum `.dat` files
- **nanonis-plot** — plot frequency vs PSD using Plotters

The workspace is designed for researchers working with Nanonis SPM output and
provides a simple workflow from data → plot.

---

## Workspace Structure
```text
.
├── Cargo.toml # Workspace root
├── README.md # This file

├── nanonis-parse/ # Crate 1: parser
│ ├── Cargo.toml
│ ├── src/lib.rs
│ └── plots/ # Example output
│ └── psd_vs_frequency.png

├── nanonis-plot/ # Crate 2: plotter
│ ├── Cargo.toml
│ ├── src/lib.rs
│ └── examples/
│ └── semilogy_plot.rs # Example plot script

└── target/ # Workspace build artifacts
```

---

## Crate: `nanonis-parse`

A simple parser for Nanonis `.dat` spectrum files.

### Features

- Read `.dat` files exported by Nanonis SPM
- Extract:
  - Frequency axis
  - Current PSD or other spectrum values
  - Header metadata
- Return strongly-typed Rust structs for downstream processing

---

## Crate: `nanonis-plot`

Plot Nanonis spectrum data using the [`plotters`](https://crates.io/crates/plotters) crate.

### Features

- Linear or logarithmic Y-axis
- Customizable plot style:
  ```rust
  pub struct PlotStyle {
      pub width: u32,
      pub height: u32,
      pub log_y: bool,
      pub line_width: u32,
      pub line_color: RGBColor,
      pub show_mesh: bool,
  }


- Automatically save the output PNG to the same directory as the input file
- Clean figure styling (large fonts, offset superscripts, frame box)

## Example: Run a semilogy (log-Y) plot

```rust
cargo run -p nanonis-plot --example semilogy_plot /path/to/Spectrum_cur001.dat
```

This produces:

`/path/to/Spectrum_cur001.png`

## Development

### Build entire workspace
```bash
cargo build
```

## Build a specific crate
```bash
cargo build -p nanonis-parse
cargo build -p nanonis-plot
```

### Run example
```bash
cargo run -p nanonis-plot --example semilogy_plot <file.dat>
```

### Future Plans

- Add more Nanonis data types (IV curves, topo images)
- Matplotlib/Python-style figure presets
- CLI tool for batch processing Nanonis files
- Output to SVG/PDF

## License

GNU General Public License v3
