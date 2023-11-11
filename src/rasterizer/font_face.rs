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

use crate::rasterizer::bitmap::Bitmap;

pub struct FontFace {
    face: owned_ttf_parser::OwnedFace,
    height: u32,
    width: u32,
}

impl FontFace {
    pub fn new(data: Vec<u8>, index: u32) -> Result<Self, owned_ttf_parser::FaceParsingError> {
        let face = owned_ttf_parser::OwnedFace::from_vec(data, index)?;
        Ok(Self { face, height: 0, width: 0 })
    }

    pub fn set_cell_pixel(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    }

    // FreeType's Load_Char API with render mode will change the glyph slot in `Face`, the result
    // `Bitmap` object can only be used before another call of load_char itself. So we consume self
    // and move it into the result `Bitmap`, which has an `return_face` method will consume itself
    // and return the `Face` to you.
    pub fn load_char(self, _c: char, _mono: bool) -> Result<Bitmap, ()> {
        // let mut flag = ft::FT_LOAD_RENDER;
        // if mono {
        //     flag |= ft::FT_LOAD_MONOCHROME;
        // }
        // let c = ft::FT_ULong::from(u32::from(c));
        // let ret = unsafe {
        //     #[allow(clippy::cast_possible_wrap)] // flag enum value is small enough
        //     ft::FT_Load_Char(self.face, c, flag as ft::FT_Int)
        // };

        // if ret == 0 {
        //     Ok(Bitmap::new(self))
        // } else {
        //     Err((self, ret))
        // }
        todo!()
    }
}
