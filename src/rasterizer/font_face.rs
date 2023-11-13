// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2020 7sDream <i@7sdre.am> and contributors
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

use ab_glyph::{Font, FontRef, GlyphId, InvalidFont};

use super::{Bitmap, PixelFormat};

pub struct FontFace<'a> {
    face: FontRef<'a>,
    height: u32,
    width: u32,
}

impl<'a> FontFace<'a> {
    pub fn new(data: &'a [u8], index: u32) -> Result<Self, InvalidFont> {
        let face = FontRef::try_from_slice_and_index(data, index)?;
        Ok(Self { face, height: 0, width: 0 })
    }

    pub fn set_cell_pixel(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    }

    pub fn load_glyph(self, gid: u16, format: PixelFormat) -> Option<Bitmap> {
        let curves = self.face.outline_glyph(GlyphId(gid).with_scale(self.height as f32))?;
        Some(Bitmap::new(&curves, format))
    }
}
