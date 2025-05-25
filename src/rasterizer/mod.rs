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

use ab_glyph::{Font, FontRef, GlyphId, InvalidFont, PxScale};

pub use self::bitmap::Bitmap;

pub struct Rasterizer<'a> {
    face: FontRef<'a>,
    height: u32,
    hscale: f32,
}

impl<'a> Rasterizer<'a> {
    pub fn new(data: &'a [u8], index: u32) -> Result<Self, InvalidFont> {
        let face = FontRef::try_from_slice_and_index(data, index)?;
        Ok(Self {
            face,
            height: 0,
            hscale: 1.0,
        })
    }

    pub fn set_pixel_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_hscale(&mut self, scale: f32) {
        self.hscale = scale
    }

    pub fn rasterize(self, gid: u16) -> Option<Bitmap> {
        let glyph_id = GlyphId(gid);
        let glyph = glyph_id.with_scale(PxScale {
            x: self.height as f32 * self.hscale,
            y: self.height as f32,
        });
        let curve = self.face.outline_glyph(glyph)?;
        Some(Bitmap::new(&curve))
    }
}
