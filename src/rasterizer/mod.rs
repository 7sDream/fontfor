// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2025 7sDream <i@7sdre.am> and contributors
//
// This file is part of FontFor.
//
// FontFor is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod bitmap;
mod pen;

use skrifa::{
    FontRef, GlyphId, MetadataProvider,
    instance::{LocationRef, Size},
    outline::DrawSettings,
};

pub use self::bitmap::Bitmap;
use self::pen::Pen;
use crate::error::Error;

pub struct Rasterizer<'a> {
    font: FontRef<'a>,
    height: u32,
    hscale: f32,
}

impl<'a> Rasterizer<'a> {
    pub fn new(data: &'a [u8], index: u32, height: u32) -> Result<Self, Error> {
        let font = FontRef::from_index(data, index)?;
        Ok(Self {
            font,
            height,
            hscale: 1.0,
        })
    }

    pub fn set_hscale(&mut self, scale: f32) {
        self.hscale = scale
    }

    pub fn rasterize(self, gid: GlyphId) -> Result<Bitmap, Error> {
        let size = Size::new(self.height as f32);
        let location = LocationRef::default();

        let metrics = self.font.glyph_metrics(size, location);
        let outlines = self.font.outline_glyphs();
        let glyph = outlines.get(gid).ok_or(Error::MissingGlyphData)?;
        let bound = metrics.bounds(gid).ok_or(Error::MissingGlyphData)?;

        let settings =
            DrawSettings::unhinted(Size::new(self.height as f32), LocationRef::default());

        let mut pen = Pen::new(bound, self.hscale);

        glyph.draw(settings, &mut pen.adapter())?;

        Ok(pen.bitmap())
    }
}
