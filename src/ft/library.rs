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
    crate::font::render::{CharRendererLoader, LoaderInput},
    freetype::freetype as ft,
    std::{hint::unreachable_unchecked, ptr},
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
}

impl<'i> CharRendererLoader<'i> for Library {
    type Render = FontFace<'i>;
    type Error = ft::FT_Error;

    fn load_render(&'i self, input: &LoaderInput<'_>) -> Result<Self::Render, Self::Error> {
        match input {
            LoaderInput::FreeType(path, index) => {
                let path = path.as_ref();
                FontFace::new(self, path, *index as ft::FT_Long)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            ft::FT_Done_Library(self.library);
        }
    }
}
