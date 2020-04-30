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
    super::{FontFace, FreeTypeError},
    freetype::freetype as ft,
    std::{path::Path, ptr},
};

pub struct Library {
    pub(super) library: ft::FT_Library,
}

impl Library {
    pub fn new() -> Result<Self, i32> {
        let mut library = ptr::null_mut();
        let ret = unsafe { ft::FT_Init_FreeType(&mut library as *mut ft::FT_Library) };
        ret.map_result(|| Self { library })
    }

    pub fn load_font<P>(&self, path: P, index: ft::FT_Long) -> Result<FontFace<'_>, ft::FT_Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        FontFace::new(self, path, index)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            ft::FT_Done_Library(self.library);
        }
    }
}
