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
        collections::HashMap,
        ffi::{CStr, CString},
        marker::PhantomData,
        os::raw::c_char,
    },
};

pub type ValueByLang<'a, T> = HashMap<&'a str, Vec<T>>;
pub type StrByLang<'a> = ValueByLang<'a, &'a str>;

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
    fn get_string_property(&self, name: &str) -> Vec<&'a str> {
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
                if let Ok(value_str) = value_str.to_str() {
                    ret.push(value_str);
                }
                n += 1;
            } else {
                break;
            }
        }
        ret
    }

    fn get_string_by_lang_property(
        &self, value_key: &str, lang_key: &str,
    ) -> Result<StrByLang<'a>, ()> {
        let values = self.get_string_property(value_key);
        let langs = self.get_string_property(lang_key);
        if values.len() == langs.len() {
            let mut ret = StrByLang::new();
            langs.into_iter().zip(values.into_iter()).for_each(|(lang, value)| {
                ret.entry(lang).or_insert_with(|| vec![]).push(value);
            });
            Ok(ret)
        } else {
            Err(())
        }
    }

    pub fn family_names(&self) -> Result<StrByLang<'a>, ()> {
        self.get_string_by_lang_property(FC_FAMILY, FC_FAMILY_LANG)
    }

    pub fn fullnames(&self) -> Result<StrByLang<'a>, ()> {
        self.get_string_by_lang_property(FC_FULLNAME, FC_FULLNAME_LANG)
    }
}
