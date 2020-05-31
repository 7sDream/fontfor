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
    super::{Bitmap, FreeTypeError, Library},
    crate::font::render::CharRenderer,
    freetype::freetype as ft,
    std::{ffi::CString, marker::PhantomData, path::Path, ptr},
};

pub struct FontFace<'ft> {
    pub(super) face: ft::FT_Face,
    phantom: PhantomData<&'ft ()>,
}

impl<'ft> FontFace<'ft> {
    pub(super) fn new<'a>(
        library: &'ft Library, path: &'a Path, index: ft::FT_Long,
    ) -> Result<Self, ft::FT_Error> {
        // TODO: Test Windows path with non-ASCII character
        let path_str = path.as_os_str().to_str().unwrap();
        let path_c_string = CString::new(path_str).unwrap();

        let mut face = ptr::null_mut();
        let ret = unsafe {
            ft::FT_New_Face(
                library.library,
                path_c_string.as_ptr(),
                index,
                &mut face as *mut ft::FT_Face,
            )
        };

        ret.as_result(Self { face, phantom: PhantomData })
    }

    #[allow(dead_code)]
    pub fn set_height_pixel(&mut self, height: ft::FT_UInt) -> Result<(), ft::FT_Error> {
        let ret = unsafe { ft::FT_Set_Pixel_Sizes(self.face, 0, height) };
        ret.as_result(())
    }

    #[allow(dead_code)]
    pub fn set_width_pixel(&mut self, width: ft::FT_UInt) -> Result<(), ft::FT_Error> {
        let ret = unsafe { ft::FT_Set_Pixel_Sizes(self.face, width, 0) };
        ret.as_result(())
    }
}

impl<'ft> CharRenderer for FontFace<'ft> {
    type Result = Bitmap<'ft>;
    type Error = ft::FT_Error;

    fn set_cell_pixel(&mut self, height: usize, width: usize) -> Result<(), ft::FT_Error> {
        let mut request = ft::FT_Size_RequestRec {
            type_: ft::FT_Size_Request_Type::FT_SIZE_REQUEST_TYPE_CELL,
            width: (width << 6) as ft::FT_Long, // This FreeType API accept number in 26.6 fixed float format
            height: (height << 6) as ft::FT_Long,
            horiResolution: 0,
            vertResolution: 0,
        };

        let ret = unsafe { ft::FT_Request_Size(self.face, &mut request as *mut _) };

        ret.as_result(())
    }

    // FreeType's Load_Char API with render mode will change the glyph slot in `Face`, the result
    // `Bitmap` object can only be used before another call of load_char itself. So we consume self
    // and move it into the result `Bitmap`, which has an `return_face` method will consume itself
    // and return the `Face` to you.
    fn render_char(self, c: char, mono: bool) -> Result<Self::Result, (Self, ft::FT_Error)> {
        let mut flag = ft::FT_LOAD_RENDER;
        if mono {
            flag |= ft::FT_LOAD_MONOCHROME;
        }
        let c = ft::FT_ULong::from(u32::from(c));
        let ret = unsafe {
            #[allow(clippy::cast_possible_wrap)] // flag enum value is small enough
            ft::FT_Load_Char(self.face, c, flag as ft::FT_Int)
        };

        if ret == 0 {
            Ok(Bitmap::new(self))
        } else {
            Err((self, ret))
        }
    }
}
