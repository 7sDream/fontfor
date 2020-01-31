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

use {super::FontFace, freetype::freetype as ft, std::os::raw};

pub struct Metrics {
    pub left: ft::FT_Int,
    pub top: ft::FT_Int,
    pub height: raw::c_uint,
    pub width: raw::c_uint,
}

pub struct Bitmap<'ft> {
    face: FontFace<'ft>,
    metrics: Metrics,
    bitmap: &'static [u8],
}

impl<'ft> Bitmap<'ft> {
    #[allow(unused_mut)]
    pub(super) fn new(mut face: FontFace<'ft>) -> Self {
        let face_rec = unsafe { &*face.face };
        let glyph = unsafe { &*face_rec.glyph };
        let left = glyph.bitmap_left;
        let top = glyph.bitmap_top;
        let width = glyph.bitmap.width;
        let height = glyph.bitmap.rows;
        let size = (width * height) as usize;
        let bitmap = unsafe { std::slice::from_raw_parts(glyph.bitmap.buffer, size) };
        Self { face, metrics: Metrics { left, top, height, width }, bitmap }
    }

    pub const fn return_face(self) -> FontFace<'ft> {
        self.face
    }

    pub const fn get_metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub const fn get_buffer(&self) -> &[u8] {
        self.bitmap
    }
}
