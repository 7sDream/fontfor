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

use core_foundation::{
    base::{kCFAllocatorDefault, CFRange, TCFType},
    characterset::{
        CFCharacterSet, CFCharacterSetAddCharactersInRange, CFCharacterSetCreateMutable,
    },
};

pub struct Charset {
    pub(super) ptr: CFCharacterSet,
}

impl Default for Charset {
    fn default() -> Self {
        let ptr = unsafe {
            CFCharacterSet::wrap_under_create_rule(CFCharacterSetCreateMutable(kCFAllocatorDefault))
        };
        Self { ptr }
    }
}

impl Charset {
    #[allow(unused_mut)] // because we add char to internal struct
    pub fn add_char(mut self, c: char) -> Self {
        #[allow(clippy::cast_possible_wrap)] // Because c is a unicode character
        let range = CFRange::init(c as u32 as isize, 1);
        unsafe { CFCharacterSetAddCharactersInRange(self.ptr.as_concrete_TypeRef(), range) }
        self
    }
}
