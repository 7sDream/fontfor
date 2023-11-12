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

use super::FontFace;

pub struct Metrics {
    pub left: usize,
    pub top: usize,
    pub height: usize,
    pub width: usize,
}

pub struct Bitmap {
    pixel_mode: u8,
    pitch: u32,
    metrics: Metrics,
    bitmap: Vec<u8>,
}

impl Bitmap {
    pub(super) fn new(_font_face: FontFace) -> Self {
        // let face_rec = unsafe { &*font_face.face };
        // let glyph = unsafe { &*face_rec.glyph };
        // let left = glyph.bitmap_left;
        // let top = glyph.bitmap_top;
        // let width = glyph.bitmap.width;
        // let height = glyph.bitmap.rows;
        // let pixel_mode = glyph.bitmap.pixel_mode;
        // let pitch = glyph.bitmap.pitch.abs() as u32;
        // let size = (pitch * height) as usize;
        // let bitmap = unsafe { std::slice::from_raw_parts(glyph.bitmap.buffer, size) };
        // Self { font_face, pixel_mode, pitch, metrics: Metrics { left, top, height, width },
        // bitmap }
        todo!()
    }

    pub const fn get_metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub fn get_pixel(&self, _row: usize, _col: usize) -> u8 {
        // if u32::from(self.pixel_mode) == ft::FT_Pixel_Mode::FT_PIXEL_MODE_MONO as u32 {
        //     let index = (row * self.pitch + col / 8) as usize;
        //     #[allow(clippy::cast_possible_truncation)] // because we mod with 8 so result is 0 -
        // 7     let bit_pos = (col % 8) as u8;
        //     let gray = self.bitmap[index];
        //     let mask = 0b_1000_0000 >> (bit_pos);
        //     if gray & mask == 0 {
        //         u8::min_value()
        //     } else {
        //         u8::max_value()
        //     }
        // } else {
        //     let index = (row * self.pitch + col) as usize;
        //     self.bitmap[index]
        // }
        todo!()
    }
}
