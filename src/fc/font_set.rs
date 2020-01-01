// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 7sDream <i@7sdre.am> and contributors
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
    super::{consts::*, Font, Pattern},
    fontconfig::fontconfig as fc,
    std::marker::PhantomData,
};

pub struct FontSet {
    ptr: *mut fc::FcFontSet,
}

impl Drop for FontSet {
    fn drop(&mut self) {
        unsafe {
            fc::FcFontSetDestroy(self.ptr);
        }
    }
}

impl FontSet {
    /// ## Safety
    ///
    /// the ptr must be
    ///
    /// - point to a valid `FcFontSet` struct
    /// - create from functions of `fontconfig` lib which do the RC thing correctly
    const unsafe fn from_ptr(ptr: *mut fc::FcFontSet) -> Self {
        Self { ptr }
    }

    pub fn match_pattern(pattern: &Pattern) -> Self {
        unsafe {
            Self::from_ptr(fc::FcFontList(std::ptr::null_mut(), pattern.ptr, THE_OBJECT_SET.ptr))
        }
    }

    pub fn fonts<'a>(&'a self) -> Fonts<'a> {
        let fs = unsafe { self.ptr.as_ref() }.unwrap();

        assert!(fs.nfont >= 0);
        #[allow(clippy::cast_sign_loss)]
        let fonts_count = fs.nfont as usize;

        let fonts_array = unsafe { std::slice::from_raw_parts::<'a>(fs.fonts, fonts_count) };

        Fonts { current: 0, fonts_array }
    }
}

pub struct Fonts<'a> {
    current: usize,
    fonts_array: &'a [*mut fc::FcPattern],
}

impl<'a> Iterator for Fonts<'a> {
    type Item = Font<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.fonts_array.len() {
            self.current += 1;
            Some(Font { ptr: self.fonts_array[self.current - 1], phantom: PhantomData })
        } else {
            None
        }
    }
}
