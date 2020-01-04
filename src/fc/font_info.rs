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

pub type ValuesByLang<'a, T> = HashMap<&'a str, Vec<T>>;
pub type StrValuesByLang<'a> = ValuesByLang<'a, &'a str>;

/// This struct is a convenient type to represent fonts in `FontSet`'s font array.
///
/// Because all inner memory will be auto freed when `FontSet` dropped, this type **DO NOT** free
/// memory of its inner `FcPattern`.
///
/// The lifetime `'a` must be smaller then corresponding `FontSet` object's.
pub struct FontInfo<'a> {
    pub(super) ptr: *mut fc::FcPattern,
    pub(super) phantom: PhantomData<&'a ()>,
}

impl<'a> FontInfo<'a> {
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

    fn get_string_by_lang_property<F>(
        &self, value_key: &str, lang_key: &str, value_map: Option<&F>,
    ) -> Result<StrValuesByLang<'a>, ()>
    where
        F: Fn(&'a str) -> &'a str,
    {
        let values = self.get_string_property(value_key);
        let languages = self.get_string_property(lang_key);
        if values.len() == languages.len() {
            let mut ret = StrValuesByLang::new();
            languages.into_iter().zip(values.into_iter()).for_each(|(lang, value)| {
                ret.entry(lang)
                    .or_insert_with(|| vec![])
                    .push(value_map.map_or(value, |f| f(value)));
            });
            Ok(ret)
        } else {
            Err(())
        }
    }

    fn remove_prefix_dot(name: &str) -> &str {
        // TODO: figure out what's the meaning of prefix dot then decide remove it or not.
        name.trim_start_matches('.')
    }

    pub fn family_names(&self) -> Result<StrValuesByLang<'a>, ()> {
        self.get_string_by_lang_property(FC_FAMILY, FC_FAMILY_LANG, Some(&Self::remove_prefix_dot))
    }

    pub fn fullnames(&self) -> Result<StrValuesByLang<'a>, ()> {
        self.get_string_by_lang_property(
            FC_FULLNAME,
            FC_FULLNAME_LANG,
            Some(&Self::remove_prefix_dot),
        )
    }
}
