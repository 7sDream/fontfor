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
    super::{Charset, FontDescriptor, FontInfo},
    crate::font::matcher::FontMatcher,
    core_foundation::{array::CFArray, base::TCFType},
    core_text::font_descriptor::{CTFontDescriptor, CTFontDescriptorCreateMatchingFontDescriptors},
};

pub struct FontSet {
    ptr: CFArray<CTFontDescriptor>,
}

impl FontSet {
    pub fn match_descriptor(desc: &FontDescriptor) -> Self {
        let ptr = unsafe {
            CFArray::<CTFontDescriptor>::wrap_under_create_rule(
                CTFontDescriptorCreateMatchingFontDescriptors(
                    desc.ptr.as_concrete_TypeRef(),
                    std::ptr::null(),
                ),
            )
        };
        Self { ptr }
    }
}

pub struct Fonts<'fs> {
    current: isize,
    array: &'fs CFArray<CTFontDescriptor>,
}

impl<'fs> Iterator for Fonts<'fs> {
    type Item = FontInfo<'fs>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.array.len() {
            self.current += 1;
            Some(FontInfo { desc: self.array.get(self.current - 1).unwrap() })
        } else {
            None
        }
    }
}

impl<'fs> FontMatcher<'fs, FontInfo<'fs>> for FontSet {
    type Output = Fonts<'fs>;

    fn init() -> Result<(), ()> {
        Ok(())
    }

    fn finalize() {}

    fn fonts_contains(c: char) -> Self {
        let charset = Charset::default().add_char(c);
        let descriptor = FontDescriptor::new_with_charset(&charset);
        Self::match_descriptor(&descriptor)
    }

    fn fonts(&'fs self) -> Self::Output {
        Fonts { current: 0, array: &self.ptr }
    }
}
