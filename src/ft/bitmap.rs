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

use {
    super::FontFace,
    crate::font::render::CharRenderResult,
    freetype::freetype as ft,
    std::{borrow::Cow, iter::Iterator, os::raw},
};

pub struct Metrics {
    pub left: ft::FT_Int,
    pub top: ft::FT_Int,
    pub height: raw::c_uint,
    pub width: raw::c_uint,
}

pub struct Bitmap<'ft> {
    font_face: FontFace<'ft>,
    metrics: Metrics,
    bitmap: Vec<Cow<'ft, [u8]>>,
}

struct U8Bits {
    index: u8,
    value: u8,
}

impl Iterator for U8Bits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index == 8 {
            None
        } else {
            self.index += 1;
            Some(if (self.value & (0b1000_0000 >> index)) == 0 {
                u8::min_value()
            } else {
                u8::max_value()
            })
        }
    }
}

const fn bits(value: u8) -> U8Bits {
    U8Bits { index: 0, value }
}

impl<'ft> Bitmap<'ft> {
    pub(super) fn new(font_face: FontFace<'ft>) -> Self {
        let face_rec = unsafe { &*font_face.face };
        let glyph = unsafe { &*face_rec.glyph };
        let left = glyph.bitmap_left;
        let top = glyph.bitmap_top;
        let width = glyph.bitmap.width;
        let height = glyph.bitmap.rows;
        let pixel_mode = glyph.bitmap.pixel_mode;
        let pitch = glyph.bitmap.pitch.abs() as usize;
        let size = pitch * height as usize;
        let bitmap = unsafe { std::slice::from_raw_parts(glyph.bitmap.buffer, size) };

        let bitmap = if u32::from(pixel_mode) == ft::FT_Pixel_Mode::FT_PIXEL_MODE_MONO as u32 {
            bitmap
                .chunks(pitch)
                .map(|row| {
                    row.iter()
                        .flat_map(|value| bits(*value))
                        .take(width as usize)
                        .collect::<Vec<_>>()
                })
                .map(Cow::Owned)
                .collect::<Vec<_>>()
        } else {
            bitmap.chunks(pitch).map(|row| Cow::from(&row[0..width as usize])).collect()
        };

        Self { font_face, metrics: Metrics { left, top, height, width }, bitmap }
    }

    pub const fn get_metrics(&self) -> &Metrics {
        &self.metrics
    }
}

impl<'ft> CharRenderResult for Bitmap<'ft> {
    type Render = FontFace<'ft>;

    fn return_render(self) -> Self::Render {
        self.font_face
    }

    fn get_height(&self) -> usize {
        self.get_metrics().height as usize
    }

    fn get_width(&self) -> usize {
        self.get_metrics().width as usize
    }

    fn get_buffer(&self) -> &[Cow<'_, [u8]>] {
        &self.bitmap
    }
}
