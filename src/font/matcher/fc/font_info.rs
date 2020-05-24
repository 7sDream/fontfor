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
    super::consts::{FC_FAMILY, FC_FAMILY_LANG, FC_FILE, FC_FULLNAME, FC_FULLNAME_LANG, FC_INDEX},
    crate::font::{Font, DEFAULT_LANG},
    fontconfig::fontconfig as fc,
    std::{
        borrow::Cow,
        collections::HashMap,
        convert::TryFrom,
        ffi::{CStr, CString},
        marker::PhantomData,
        os::raw::{c_char, c_int},
    },
};

/// Convenient trait for quickly get property value in default language
pub trait GetValueByLang {
    type Item;

    fn get_by_lang(&self, lang: &str) -> Option<&Self::Item>;

    fn when_missing(&self) -> &Self::Item;

    fn get_default(&self) -> &Self::Item {
        if let Some(value) = self.get_by_lang(DEFAULT_LANG) {
            value
        } else {
            self.when_missing()
        }
    }
}

pub type ValuesByLang<'a, T> = HashMap<&'a str, Vec<T>>;
pub type StrValuesByLang<'a> = ValuesByLang<'a, &'a str>;

impl<'a, T> GetValueByLang for ValuesByLang<'a, T> {
    type Item = T;

    fn get_by_lang(&self, lang: &str) -> Option<&Self::Item> {
        self.get(lang).and_then(|values| values.first())
    }

    fn when_missing(&self) -> &Self::Item {
        self.values().next().unwrap().first().unwrap()
    }
}

/// This struct is a convenient type to represent fonts in `FontSet`'s font array.
///
/// Because all inner memory will be auto freed when `FontSet` dropped, this type **DO NOT** free
/// memory of its inner `FcPattern`.
///
/// The lifetime `'font` must be smaller then corresponding `FontSet` object's.
pub struct FontInfo<'font> {
    pub(super) ptr: *mut fc::FcPattern,
    pub(super) phantom: PhantomData<&'font ()>,
}

impl<'font> FontInfo<'font> {
    fn get_string_property(&self, name: &str) -> Vec<&'font str> {
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
    ) -> Result<StrValuesByLang<'font>, ()>
    where
        F: Fn(&'font str) -> &'font str,
    {
        let values = self.get_string_property(value_key);
        let languages = self.get_string_property(lang_key);
        if values.len() == languages.len() {
            let mut ret = StrValuesByLang::new();
            languages.into_iter().zip(values.into_iter()).for_each(|(lang, value)| {
                ret.entry(lang)
                    .or_insert_with(Vec::new)
                    .push(value_map.map_or(value, |f| f(value)));
            });
            Ok(ret)
        } else {
            Err(())
        }
    }

    fn get_int_property(&self, name: &str) -> Vec<c_int> {
        let c_name = CString::new(name).unwrap();
        let mut ret = vec![];
        let mut n = 0;
        loop {
            let mut value = 0;
            let result = unsafe {
                fc::FcPatternGetInteger(self.ptr, c_name.as_ptr(), n, &mut value as *mut c_int)
            };
            if result == fc::FcResultMatch {
                ret.push(value);
                n += 1;
            } else {
                break;
            }
        }
        ret
    }

    fn remove_prefix_dot(name: &str) -> &str {
        // TODO: figure out what's the meaning of prefix dot then decide remove it or not.
        // Only seen this prefix dot on macOs, as we will change to use Core Text on macOS,
        // maybe this can be removed someday.
        name.trim_start_matches('.')
    }

    pub fn family_names(&self) -> Result<StrValuesByLang<'font>, ()> {
        self.get_string_by_lang_property(FC_FAMILY, FC_FAMILY_LANG, Some(&Self::remove_prefix_dot))
    }

    pub fn fullnames(&self) -> Result<StrValuesByLang<'font>, ()> {
        self.get_string_by_lang_property(
            FC_FULLNAME,
            FC_FULLNAME_LANG,
            Some(&Self::remove_prefix_dot),
        )
    }

    pub fn path(&self) -> Result<&'font str, ()> {
        self.get_string_property(FC_FILE).pop().ok_or(())
    }

    pub fn index(&self) -> Result<c_int, ()> {
        self.get_int_property(FC_INDEX).pop().ok_or(())
    }
}

impl<'fi> TryFrom<FontInfo<'fi>> for Font<'fi> {
    type Error = ();

    fn try_from(font_info: FontInfo<'fi>) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_sign_loss)] // Because it is index
        let f = Self {
            family_name: Cow::from(*font_info.family_names()?.get_default()),
            fullname: Cow::from(*font_info.fullnames()?.get_default()),
            path: Cow::from(font_info.path()?),
            index: font_info.index()? as usize,
        };
        if f.family_name.is_empty() || f.fullname.is_empty() {
            Err(())
        } else {
            Ok(f)
        }
    }
}
