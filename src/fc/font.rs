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
    super::consts::*,
    fontconfig::fontconfig as fc,
    std::{
        ffi::{CStr, CString},
        marker::PhantomData,
        os::raw::c_char,
    },
};

/// This struct is a convenient type to represent fonts in `FontSet`'s font array.
///
/// Because all inner memory will be auto freed when `FontSet` dropped, this type **DO NOT** free
/// memory of its inner `FcPattern`.
///
/// The lifetime `'a` must be smaller then corresponding `FontSet` object's.
pub struct Font<'a> {
    pub(super) ptr: *mut fc::FcPattern,
    pub(super) phantom: PhantomData<&'a ()>,
}

impl<'a> Font<'a> {
    fn get_string_property(&self, name: &str) -> Vec<String> {
        let c_name = CString::new(name).unwrap();
        let mut ret = vec![];
        let mut n = 0;
        loop {
            let mut value: *mut u8 = std::ptr::null_mut();
            let result = unsafe {
                fc::FcPatternGetString(self.ptr, c_name.as_ptr(), n, &mut value as *mut *mut u8)
            };
            if result == fc::FcResultMatch {
                let value_str = unsafe { CStr::from_ptr(value as *mut c_char) };
                ret.push(value_str.to_string_lossy().to_string());
                n += 1;
            } else {
                break;
            }
        }
        ret
    }

    pub fn family(&self) -> Vec<String> {
        self.get_string_property(FC_FAMILY)
    }

    #[allow(dead_code)]
    pub fn fullname(&self) -> Vec<String> {
        self.get_string_property(FC_FULLNAME)
    }
}
