use crate::classes::c_color_box::ColorBox16;
use crate::classes::c_rgb16::Rgb16;

pub fn build_palette_median_cut_rgba16(src16: &[u16], w: usize, h: usize, k: usize) -> Vec<Rgb16> {
    // src16: [r,g,b,a, r,g,b,a ...], len = w*h*4
    assert_eq!(src16.len(), w * h * 4);

    let samples = sample_rgb16_from_rgba16(src16, w, h, 50_000, 512); // max_samples, alpha_threshold(u16)
    median_cut_palette(samples, k.max(2))
}

pub fn sample_rgb16_from_rgba16(
    src16: &[u16],
    w: usize,
    h: usize,
    max_samples: usize,
    alpha_threshold: u16,
) -> Vec<Rgb16> {
    let total = w * h;
    let step = (total / max_samples).max(1);

    let mut out = Vec::with_capacity(total.min(max_samples));
    for i in (0..total).step_by(step) {
        let p = i * 4;
        let a = src16[p + 3];
        if a <= alpha_threshold {
            continue;
        }
        out.push(Rgb16 {
            r: src16[p],
            g: src16[p + 1],
            b: src16[p + 2],
        });
    }

    if out.is_empty() && total > 0 {
        out.push(Rgb16 { r: src16[0], g: src16[1], b: src16[2] });
    }
    out
}


pub fn median_cut_palette(mut samples: Vec<Rgb16>, k: usize) -> Vec<Rgb16> {
    if samples.is_empty() {
        return vec![Rgb16 { r: 0, g: 0, b: 0 }];
    }

    let mut boxes = vec![ColorBox16 { colors: samples }];

    while boxes.len() < k {
        let (best_i, _) = boxes
            .iter()
            .enumerate()
            .max_by_key(|(_, b)| (b.score(), b.colors.len()))
            .unwrap();

        let b = boxes.remove(best_i);
        let Some((b1, b2)) = b.clone().split() else {
            boxes.push(b);
            break;
        };

        boxes.push(b1);
        boxes.push(b2);
    }

    boxes.into_iter().map(|b| b.average()).collect()
}


pub fn u16_to_u8(v: u16) -> u8 {
    ((v as u32 + 128) / 257) as u8   // check1 overflow
}

pub fn nearest_palette_color(c: Rgb16, palette: &[Rgb16]) -> Rgb16 {
    let mut best = palette[0];
    let mut best_d: u64 = u64::MAX;

    for &p in palette {
        let dr = c.r as i32 - p.r as i32;
        let dg = c.g as i32 - p.g as i32;
        let db = c.b as i32 - p.b as i32;
        let d = (dr as i64 * dr as i64 + dg as i64 * dg as i64 + db as i64 * db as i64) as u64;
        if d < best_d {
            best_d = d;
            best = p;
        }
    }
    best
}

pub fn clamp_u16_i32(v: i32) -> u16 {
    if v < 0 { 0 } else if v > 65535 { 65535 } else { v as u16 }
}



pub fn dither_fs_palette_rgba16_to_rgba8(
    src16: &[u16],
    dst8: &mut Vec<u8>,
    w: usize,
    h: usize,
    palette: &[Rgb16],
) {
    assert_eq!(src16.len(), w * h * 4);
    dst8.resize(w * h * 4, 0);

    let mut er0 = vec![0i32; w + 2];
    let mut eg0 = vec![0i32; w + 2];
    let mut eb0 = vec![0i32; w + 2];
    let mut er1 = vec![0i32; w + 2];
    let mut eg1 = vec![0i32; w + 2];
    let mut eb1 = vec![0i32; w + 2];

    for y in 0..h {
        er1.fill(0); eg1.fill(0); eb1.fill(0);

        for x in 0..w {
            let p = (y * w + x) * 4;

            let r = clamp_u16_i32(src16[p] as i32 + er0[x]);
            let g = clamp_u16_i32(src16[p + 1] as i32 + eg0[x]);
            let b = clamp_u16_i32(src16[p + 2] as i32 + eb0[x]);
            let a = src16[p + 3];

            let cur = Rgb16 { r, g, b };
            let q = nearest_palette_color(cur, palette);

            dst8[p]     = u16_to_u8(q.r);
            dst8[p + 1] = u16_to_u8(q.g);
            dst8[p + 2] = u16_to_u8(q.b);
            dst8[p + 3] = u16_to_u8(a);

            let err_r = cur.r as i32 - q.r as i32;
            let err_g = cur.g as i32 - q.g as i32;
            let err_b = cur.b as i32 - q.b as i32;

            // Floyd–Steinberg weights (divide on 16)
            // right: 7/16
            if x + 1 < w {
                er0[x + 1] += (err_r * 7) / 16;
                eg0[x + 1] += (err_g * 7) / 16;
                eb0[x + 1] += (err_b * 7) / 16;
            }
            // down-left: 3/16
            if x > 0 && y + 1 < h {
                er1[x - 1] += (err_r * 3) / 16;
                eg1[x - 1] += (err_g * 3) / 16;
                eb1[x - 1] += (err_b * 3) / 16;
            }
            // down: 5/16
            if y + 1 < h {
                er1[x] += (err_r * 5) / 16;
                eg1[x] += (err_g * 5) / 16;
                eb1[x] += (err_b * 5) / 16;
            }
            // down-right: 1/16
            if x + 1 < w && y + 1 < h {
                er1[x + 1] += (err_r * 1) / 16;
                eg1[x + 1] += (err_g * 1) / 16;
                eb1[x + 1] += (err_b * 1) / 16;
            }
        }

        // next -> current
        std::mem::swap(&mut er0, &mut er1);
        std::mem::swap(&mut eg0, &mut eg1);
        std::mem::swap(&mut eb0, &mut eb1);
    }
}

const BAYER8: [[u8; 8]; 8] = [
    [0, 48, 12, 60, 3, 51, 15, 63],
    [32, 16, 44, 28, 35, 19, 47, 31],
    [8, 56, 4, 52, 11, 59, 7, 55],
    [40, 24, 36, 20, 43, 27, 39, 23],
    [2, 50, 14, 62, 1, 49, 13, 61],
    [34, 18, 46, 30, 33, 17, 45, 29],
    [10, 58, 6, 54, 9, 57, 5, 53],
    [42, 26, 38, 22, 41, 25, 37, 21],
];


pub fn dither_ordered_levels_rgba16_to_rgba8(
    src16: &[u16],
    dst8: &mut [u8],
    w: usize,
    h: usize,
    levels: u16,
) {
    debug_assert_eq!(src16.len(), w * h * 4);
    debug_assert_eq!(dst8.len(), w * h * 4);

    let max_q = (levels - 1) as f32;

    for y in 0..h {
        for x in 0..w {
            let p = (y * w + x) * 4;

            // step 0..1
            let t = (BAYER8[y & 7][x & 7] as f32 + 0.5) / 64.0;

            for c in 0..4 {
                let v16 = src16[p + c] as f32 / 65535.0; // 0..1
                let q = v16 * max_q;                     // 0..(levels-1)
                let base = q.floor();
                let frac = q - base;

                let mut qi = base + if frac > t { 1.0 } else { 0.0 };
                if qi < 0.0 { qi = 0.0; }
                if qi > max_q { qi = max_q; }

                let out16 = (qi / max_q) * 65535.0;
                let out16u = (out16 + 0.5) as u16;

                // u16 -> u8
                dst8[p + c] = (out16u >> 8) as u8;
            }
        }
    }
}


pub fn dither_ordered_levels_rgba8_inplace(
    rgba: &mut [u8],
    w: usize,
    h: usize,
    levels: u8,
) {
    debug_assert_eq!(rgba.len(), w * h * 4);

    let levels = levels.max(2);
    let max_q = (levels - 1) as f32;

    for y in 0..h {
        for x in 0..w {
            let p = (y * w + x) * 4;

            // step 0..1
            let t = (BAYER8[y & 7][x & 7] as f32 + 0.5) / 64.0;

            for c in 0..4 {
                let v = rgba[p + c] as f32 / 255.0; // 0..1
                let q = v * max_q;
                let base = q.floor();
                let frac = q - base;

                let mut qi = base + if frac > t { 1.0 } else { 0.0 };
                if qi < 0.0 { qi = 0.0; }
                if qi > max_q { qi = max_q; }

                let out = (qi / max_q) * 255.0;
                rgba[p + c] = (out + 0.5) as u8;
            }
        }
    }
}


/// Nearest resize for interleaved pixelx: [c0,c1,c2,(c3), c0,c1,c2,(c3), ...]
/// T: u8 or u16
/// C: channel size (3=RGB, 4=RGBA)
pub fn resize_interleaved_nearest<T: Copy + Default, const C: usize>(
    src: &[T],
    w: usize,
    h: usize,
    scale: f32,
) -> (Vec<T>, usize, usize) {
    assert!(C > 0);
    assert_eq!(src.len(), w * h * C);

    let scale = scale.max(0.0001);
    let new_w = ((w as f32) * scale).round().max(1.0) as usize;
    let new_h = ((h as f32) * scale).round().max(1.0) as usize;

    let mut dst = vec![T::default(); new_w * new_h * C];

    for y in 0..new_h {
        let sy = y * h / new_h;
        for x in 0..new_w {
            let sx = x * w / new_w;

            let si = (sy * w + sx) * C;
            let di = (y * new_w + x) * C;

            dst[di..di + C].copy_from_slice(&src[si..si + C]);
        }
    }

    (dst, new_w, new_h)
}


pub fn rgb16_to_color32(c: Rgb16) -> egui::Color32 {
    egui::Color32::from_rgb((c.r >> 8) as u8, (c.g >> 8) as u8, (c.b >> 8) as u8)
}

pub fn color32_to_rgb16(c: egui::Color32) -> Rgb16 {
    let [r,g,b,_a] = c.to_srgba_unmultiplied();
    Rgb16 { r: (r as u16) * 257, g: (g as u16) * 257, b: (b as u16) * 257 }
}

pub fn rgb16_to_u8(c: Rgb16) -> (u8, u8, u8) {
    ((c.r >> 8) as u8, (c.g >> 8) as u8, (c.b >> 8) as u8)
}

pub fn rgb16_to_u8_exact(c: Rgb16) -> (u8, u8, u8) {
    (u16_to_u8(c.r), u16_to_u8(c.g), u16_to_u8(c.b))
}

pub fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}


pub fn set_texture(
    tex: &mut Option<egui::TextureHandle>,
    ctx: &egui::Context,
    name: &str,
    w: usize,
    h: usize,
    rgba8: &[u8],
) {
    let img = egui::ColorImage::from_rgba_unmultiplied([w, h], rgba8);
    match tex {
        Some(t) => t.set(img, egui::TextureOptions::NEAREST),
        None => *tex = Some(ctx.load_texture(name, img, egui::TextureOptions::NEAREST)),
    }
}

pub fn rgba8_to_rgba16(src: &[u8], dst: &mut Vec<u16>) {
    dst.resize(src.len(), 0);
    for (i, &v) in src.iter().enumerate() {
        dst[i] = (v as u16) * 257; // 0..255 -> 0..65535
    }
}