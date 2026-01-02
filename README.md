# Dithering Slave (Rust + eframe/egui)

A small desktop tool for **palette-based dithering** and **palette editing** written in Rust **without a heavy framework**: image decoding/encoding, palette extraction (Median Cut), Floyd–Steinberg dithering, and a simple GUI built with **eframe/egui**.

This project is a practical playground for Rust + realtime-ish image processing with a minimal, hackable codebase.

<b>Created without vibecoding</b>

---

## Screenshots

<p float="left">
  <img src="/screenshots/Screenshot_1.png" width="49%" />
  -
  <img src="/screenshots/Screenshot_2.png" width="49%" />
</p>

> Put your screenshots into `/screenshots/` and update the filenames above.

---

## Features

- GUI:
  - Native desktop window via **eframe/egui**
  - Top menu (Open/Save/Exit, palette actions, etc.)
  - Central preview with **Contain** scaling (image stays centered on resize)
  - Bottom palette bar with clickable color swatches **(Click to change colors)**
- Image pipeline:
  - Load image from file (PNG/JPG/etc.)
  - Optional scale on load (e.g. 0.5×) for faster processing
  - Work internally with **RGBA16** for processing + **RGBA8** for display
- Palette:
  - Extract palette using **Median Cut** (configurable size: 8/16/etc.)
  - Palette override (edit colors without rebuilding the palette)
  - Replace palette color → update the preview instantly
- Dithering:
  - Floyd–Steinberg dithering to a fixed palette
  - Toggle **Original / Dithered**
- Export:
  - Save dithered output as **PNG** or etc.
---

## Tech Stack / Dependencies

- `eframe`, `egui` — GUI
- `image` — decode/encode images
- `rfd` — native file dialogs (Open/Save)
- `serde`, `toml/json` — config persistence (if you add it)

---

## Build & Run

### Requirements
- Rust (stable recommended)
- Cargo

### Run (debug)
```bash
cargo run
