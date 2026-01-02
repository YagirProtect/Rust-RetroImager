# Dithering Slave

A small desktop utility built with **Rust + eframe/egui** for **palette-based dithering** and **palette reduction**.

## Features

- Open images (PNG/JPG/etc.)
- Optional image scaling on load (e.g. 0.5×) to speed up processing
- Palette generation (Median Cut) with configurable palette size (e.g. 8/16 colors)
- Palette dithering (Floyd–Steinberg, palette-based)
- Toggle **Original / Dithered**
- Palette preview as color swatches
- Replace/override palette colors and instantly re-apply to the output
- Save the result as **PNG**

## Screenshots

- `assets/screenshot_1.png`
- `assets/screenshot_2.png`

## Build & Run

### Requirements
- Rust (stable)

### Run (debug)
```bash
cargo run
```

### Build (release)
```bash
cargo build --release
```

## Usage (UI)

Typical flow:

1. **File → Open** — load an image  
2. (Optional) adjust **Scale** / **Palette size**
3. Apply dithering (or it runs automatically depending on your UI)
4. Use the **Palette** swatches to override colors
5. **File → Save** — export the dithered PNG

## Project layout (example)

```text
src/
  main.rs
  classes/
    c_app.rs
    c_dithered_image.rs
    c_palette_menu.rs
    c_change_color_window.rs
    c_config.rs
    c_rgb16.rs
    ....
  image_utils.rs
assets/
  icon.png
```

## Tech stack

- **eframe/egui** — UI
- **image** — decoding/encoding images
- **rfd** — native file dialogs

## Performance notes

- For large images, a smaller `image_percent` / scale factor will significantly reduce processing time.
- Avoid recreating GPU textures every update; prefer `TextureHandle::set(...)` for updates to prevent stutters.

## License

Pick a license and add a `LICENSE` file (e.g. MIT or Apache-2.0).
