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

use crate::ft::bitmap::Bitmap;
use {
    super::{FreeTypeError, Library},
    freetype::freetype as ft,
    std::{ffi::CString, marker::PhantomData, path::Path, ptr},
};

pub struct FontFace<'ft> {
    pub(super) face: ft::FT_Face,
    phantom: PhantomData<&'ft ()>,
}

impl<'ft> FontFace<'ft> {
    pub(super) fn new(
        library: &'ft mut Library, path: &Path, index: ft::FT_Long,
    ) -> Result<Self, ft::FT_Error> {
        // TODO: Test windows path with non-ASCII character
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

    pub fn set_height_pixel(&mut self, height: ft::FT_UInt) -> Result<(), ft::FT_Error> {
        let ret = unsafe { ft::FT_Set_Pixel_Sizes(self.face, 0, height) };
        ret.as_result(())
    }

    pub fn load_char(self, c: char) -> Result<Bitmap<'ft>, ft::FT_Error> {
        let ret = unsafe {
            #[allow(clippy::cast_possible_wrap)]
            ft::FT_Load_Char(
                self.face,
                ft::FT_ULong::from(u32::from(c)),
                ft::FT_LOAD_RENDER as ft::FT_Int,
            )
        };
        ret.map_result(|| Bitmap::new(self))
    }
}
