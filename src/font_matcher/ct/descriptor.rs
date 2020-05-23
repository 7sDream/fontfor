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
    super::charset::Charset,
    core_foundation::{base::TCFType, dictionary::CFDictionary, string::CFString},
    core_text::font_descriptor::{
        kCTFontCharacterSetAttribute, new_from_attributes, CTFontDescriptor,
    },
};

pub struct Descriptor {
    pub(super) ptr: CTFontDescriptor,
}

impl Descriptor {
    pub fn new_with_charset(charset: &Charset) -> Self {
        let charset_key = unsafe { CFString::wrap_under_get_rule(kCTFontCharacterSetAttribute) };
        let charset_value = charset.ptr.as_CFType();
        let attritubes = CFDictionary::from_CFType_pairs(&[(charset_key, charset_value)]);
        let ptr = new_from_attributes(&attritubes);
        Self { ptr }
    }
}
