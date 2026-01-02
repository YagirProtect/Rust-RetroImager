use crate::classes::c_rgb16::Rgb16;

#[derive(Clone)]
pub struct ColorBox16 {
    pub(crate) colors: Vec<Rgb16>,
}

impl ColorBox16 {
    pub(crate) fn score(&self) -> u32 {
        let (rmin, rmax, gmin, gmax, bmin, bmax) = self.ranges();
        let dr = (rmax - rmin) as u32;
        let dg = (gmax - gmin) as u32;
        let db = (bmax - bmin) as u32;
        dr.max(dg).max(db)
    }

    fn ranges(&self) -> (u16, u16, u16, u16, u16, u16) {
        let mut rmin = u16::MAX; let mut rmax = 0;
        let mut gmin = u16::MAX; let mut gmax = 0;
        let mut bmin = u16::MAX; let mut bmax = 0;

        for c in &self.colors {
            rmin = rmin.min(c.r); rmax = rmax.max(c.r);
            gmin = gmin.min(c.g); gmax = gmax.max(c.g);
            bmin = bmin.min(c.b); bmax = bmax.max(c.b);
        }
        (rmin, rmax, gmin, gmax, bmin, bmax)
    }

    pub(crate) fn average(&self) -> Rgb16 {
        let mut sr: u64 = 0;
        let mut sg: u64 = 0;
        let mut sb: u64 = 0;
        let n = self.colors.len().max(1) as u64;

        for c in &self.colors {
            sr += c.r as u64;
            sg += c.g as u64;
            sb += c.b as u64;
        }

        Rgb16 {
            r: (sr / n) as u16,
            g: (sg / n) as u16,
            b: (sb / n) as u16,
        }
    }

    pub(crate) fn split(mut self) -> Option<(ColorBox16, ColorBox16)> {
        if self.colors.len() < 2 { return None; }

        let (rmin, rmax, gmin, gmax, bmin, bmax) = self.ranges();
        let dr = rmax - rmin;
        let dg = gmax - gmin;
        let db = bmax - bmin;

        if dr >= dg && dr >= db {
            self.colors.sort_unstable_by_key(|c| c.r);
        } else if dg >= db {
            self.colors.sort_unstable_by_key(|c| c.g);
        } else {
            self.colors.sort_unstable_by_key(|c| c.b);
        }

        let mid = self.colors.len() / 2;
        let right = self.colors.split_off(mid);
        Some((ColorBox16 { colors: self.colors }, ColorBox16 { colors: right }))
    }
}