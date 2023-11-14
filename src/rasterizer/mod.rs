// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2023 7sDream <i@7sdre.am> and contributors
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

use ab_glyph::{Font, FontRef, GlyphId, InvalidFont, OutlinedGlyph, PxScaleFactor, ScaleFont};

pub use self::bitmap::{Bitmap, Metrics};

pub struct Rasterizer<'a> {
    face: FontRef<'a>,
    height: u32,
    width: u32,
    scale: PxScaleFactor,
}

impl<'a> Rasterizer<'a> {
    pub fn new(data: &'a [u8], index: u32) -> Result<Self, InvalidFont> {
        let face = FontRef::try_from_slice_and_index(data, index)?;
        Ok(Self {
            face,
            height: 0,
            width: 0,
            scale: PxScaleFactor { horizontal: 1.0, vertical: 1.0 },
        })
    }

    pub fn set_size(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    }

    #[allow(dead_code)]
    pub fn set_scale_factor(&mut self, scale: PxScaleFactor) {
        self.scale = scale
    }

    pub fn rasterize(self, gid: u16) -> Option<Bitmap> {
        let gid = GlyphId(gid);
        let outline = self.face.outline(gid)?;

        let glyph = gid.with_scale(self.height as f32);
        let mut scale = self.face.as_scaled(glyph.scale).scale_factor();
        scale.horizontal *= self.scale.horizontal;
        scale.vertical *= self.scale.vertical;
        let curve = OutlinedGlyph::new(glyph, outline, scale);

        Some(Bitmap::new(&curve))
    }
}
