use crate::{
    tables::{TableDirectoryRepr, cmap::GlyphId},
    types::{FWORD, UFWORD, int16},
};
use std::{fmt, iter::FusedIterator};

#[repr(C)]
#[non_exhaustive]
pub struct HmtxTableRepr {
    // Note: It's a little bit faster to work with `&[FWORD]` than with separately typed slices.
    // See [`HmtxTableHandle::metric`] method for explanation.
    raw_metrics: [FWORD; 0],
    // : h_metrics: [LongHorMetricRepr; num_h_metrics]
    // : left_side_bearings: [FWORD; num_glyphs - num_h_metrics]
}

#[repr(C)]
pub struct LongHorMetricRepr {
    pub advance_width: UFWORD,
    pub lsb: FWORD,
}

// Note: HmtxTableRepr can't provide anything on its own. We need data from two other tables:
// `number_of_h_metrics` from 'hhea' and `num_glyphs` from 'maxp' to slice the data correctly.
#[derive(Copy)]
#[derive_const(Clone)]
pub struct HmtxTableHandle<'a> {
    raw_metrics: &'a [FWORD],
    num_h_metrics: usize,
}

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LongHorMetric {
    pub advance_width: u16,
    pub lsb: i16,
}

impl LongHorMetric {
    pub const fn new(advance_width: u16, lsb: i16) -> Self {
        Self { advance_width, lsb }
    }
}
const impl From<&LongHorMetricRepr> for LongHorMetric {
    fn from(value: &LongHorMetricRepr) -> Self {
        Self { advance_width: value.advance_width.get(), lsb: value.lsb.get() }
    }
}

impl<'a> HmtxTableHandle<'a> {
    pub fn new(dir: &'a TableDirectoryRepr) -> Self {
        let raw_metrics = dir.table::<HmtxTableRepr>().unwrap().raw_metrics.as_ptr();
        let num_h_metrics = dir.hhea().number_of_h_metrics.get() as usize;
        let num_glyphs = dir.maxp().num_glyphs.get() as usize;

        let total_word_count = num_h_metrics + num_glyphs;
        let raw_metrics = unsafe { std::slice::from_raw_parts(raw_metrics, total_word_count) };

        Self { raw_metrics, num_h_metrics }
    }

    pub const fn num_h_metrics(&self) -> u16 {
        self.num_h_metrics as u16
    }
    pub const fn num_glyphs(&self) -> u16 {
        (self.raw_metrics.len() - self.num_h_metrics) as u16
    }

    const fn h_metrics(&self) -> &'a [LongHorMetricRepr] {
        unsafe { std::slice::from_raw_parts(self.raw_metrics.as_ptr().cast(), self.num_h_metrics) }
    }
    const fn lsbs(&self) -> &'a [FWORD] {
        unsafe {
            std::slice::from_raw_parts(
                self.raw_metrics.as_ptr().add(self.num_h_metrics * 2),
                self.raw_metrics.len() - self.num_h_metrics * 2,
            )
        }
    }

    pub const fn last_advance_width(&self) -> Option<u16> {
        Some(self.h_metrics().last()?.advance_width.get())
    }

    pub const fn metric(&self, glyph_id: GlyphId) -> Option<LongHorMetric> {
        /// Normally, you'd check if you need to access h_metrics() or lsbs(), and then either:
        /// a) get both values from h_metrics(), or b) get lsb from lsbs(), and also maybe get
        /// the advance from h_metrics().last(), with both of these operations involving bounds
        /// checks. That's a total of 3 branches!
        ///
        /// But there's a way to combine 2 of them, leaving only 2 bounds checks:
        ///
        /// ```rs
        /// idx <= hcount-1 {
        ///     let min = idx;
        ///     // (idx*2, idx*2+1)
        ///     // (idx*2, idx+idx+1)
        ///     (min*2, min+idx+1)
        /// }
        /// idx > hcount-1 {
        ///     let min = hcount-1;
        ///     // ((hcount-1)*2, (hcount*2)+(idx-hcount))
        ///     // ((hcount-1)*2, (hcount-1)+idx+1)
        ///     (min*2, min+idx+1)
        /// }
        ///
        /// // No branching! 😎 (compiles to asm 'cmp, cmovge')
        /// let min = idx.min(hcount-1);
        /// (min*2, min+idx+1)
        /// ```
        ///
        /// Now we only have 2 bounds checks: one to ensure the glyph is in range of this cmap,
        /// and another checking if `hcount` is 0 - the only scenario in which `min` would be -1,
        /// out of range. That, of course, would mean than `min` in `min+idx+1` is `-1` too, but
        /// that's okay, - `min+1` would wrap around to 0, and all that'd remain would be `idx`.
        ///
        struct _CodeExplanation;

        let idx: usize = glyph_id.into();
        // Do the comparison as `isize`, to ensure that `-1` from `hcount-1` goes through to `min`
        let min = (idx as isize).min(self.num_h_metrics.wrapping_sub(1) as isize) as usize;

        Some(LongHorMetric {
            // Do a bounds check on min+idx+1 to check if this glyph is even represented here
            lsb: self.raw_metrics.get(min.wrapping_add(idx).wrapping_add(1))?.get(),

            advance_width: {
                if self.num_h_metrics != 0 {
                    // Unless hcount is 0, min*2 is always in valid range
                    unsafe { self.raw_metrics.get_unchecked(min.wrapping_mul(2)) }.get() as u16
                } else {
                    // Otherwise, return 0 as advance_width
                    0
                }
            },
        })
    }

    pub const fn iter(&self) -> Iter<'_> {
        Iter::new(*self)
    }
}

const impl<'a> IntoIterator for HmtxTableHandle<'a> {
    type Item = (GlyphId, LongHorMetric);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
const impl<'a> IntoIterator for &HmtxTableHandle<'a> {
    type Item = (GlyphId, LongHorMetric);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(*self)
    }
}

// TODO: When std::slice::Iter's Clone is constified, replace this with #[derive_const]
#[derive(Clone)]
pub struct Iter<'a> {
    glyph_id: u16,
    default_aw: u16,
    h_metrics: std::slice::Iter<'a, LongHorMetricRepr>,
    lsbs: std::slice::Iter<'a, int16>,
}

impl<'a> Iter<'a> {
    pub const fn new(hmtx: HmtxTableHandle<'a>) -> Self {
        Self {
            glyph_id: 0,
            default_aw: hmtx.last_advance_width().unwrap_or(0),
            h_metrics: hmtx.h_metrics().iter(),
            lsbs: hmtx.lsbs().iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (GlyphId, LongHorMetric);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(h_metric) = self.h_metrics.next() {
            let id = GlyphId::from(self.glyph_id);
            self.glyph_id += 1;

            Some((id, h_metric.into()))
        } else if let Some(lsb) = self.lsbs.next() {
            let id = GlyphId::from(self.glyph_id);
            self.glyph_id += 1;

            Some((id, LongHorMetric::new(self.default_aw, lsb.get())))
        } else {
            None
        }
    }

    fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: std::ops::Try<Output = B>,
    {
        for h_metric in &mut self.h_metrics {
            let id = GlyphId::from(self.glyph_id);
            self.glyph_id += 1;
            init = f(init, (id, h_metric.into()))?;
        }
        for lsb in &mut self.lsbs {
            let id = GlyphId::from(self.glyph_id);
            self.glyph_id += 1;
            init = f(init, (id, LongHorMetric::new(self.default_aw, lsb.get())))?;
        }

        R::from_output(init)
    }
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.try_fold(init, |init, x| Ok::<_, !>(f(init, x))).unwrap()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.h_metrics.len() + self.lsbs.len()
    }
}
impl<'a> FusedIterator for Iter<'a> {}

impl<'a> fmt::Debug for HmtxTableHandle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HmtxTable")
            .field("number_of_h_metrics", &self.num_h_metrics())
            .field("num_glyphs", &self.num_glyphs())
            .field_with("aws_lsbs", |f| {
                let mut f = f.with_options(*f.options().alternate(false));

                let mut builder = f.debug_map();
                let last = self.num_glyphs() - 1;
                for (glyph, metric) in self.iter() {
                    struct GlyphWrapper(GlyphId);
                    struct EntryWrapper(LongHorMetric, bool);

                    impl fmt::Debug for GlyphWrapper {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            if self.0.get().is_multiple_of(8) {
                                f.write_str("\n    ")?;
                            }
                            self.0.fmt(f)
                        }
                    }
                    impl fmt::Debug for EntryWrapper {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            (self.0.advance_width, self.0.lsb).fmt(f)?;
                            if self.1 { f.write_str("\n") } else { Ok(()) }
                        }
                    }

                    builder.entry(&GlyphWrapper(glyph), &EntryWrapper(metric, glyph.get() == last));
                }

                builder.finish()
            })
            .finish()
    }
}
