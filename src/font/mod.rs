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

#[cfg(target_os = "linux")]
mod from_fc;

#[cfg(target_os = "macos")]
mod from_ct;

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    ops::Deref,
};

const DEFAULT_LANG: &str = "en";

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

pub struct Family<'fs> {
    pub name: StrValuesByLang<'fs>,
    pub fonts: BinaryHeap<Reverse<Font<'fs>>>,
    pub default_name_width: usize,
}

impl<'fs> Family<'fs> {
    pub fn new(name: StrValuesByLang<'fs>) -> Self {
        let default_name = *name.get_default();
        let default_name_width = default_name.len();
        Self { name, fonts: BinaryHeap::new(), default_name_width }
    }

    pub fn styles_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn add_font(&mut self, font: Font<'fs>) -> &mut Self {
        self.fonts.push(Reverse(font));
        self
    }
}

#[derive(Eq)]
pub struct Font<'fi> {
    pub family_names: StrValuesByLang<'fi>,
    pub fullnames: StrValuesByLang<'fi>,
    pub path: &'fi str,
    pub index: usize,
}

impl<'fi> PartialEq for Font<'fi> {
    fn eq(&self, other: &Self) -> bool {
        self.fullnames.get_default() == other.fullnames.get_default()
    }
}

/// Implement `Ord` trait for store `FontInfo` in `BinaryHeap` struct
///
/// We sort font by it's fullname of default language(en).
impl<'fi> Ord for Font<'fi> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_name = *self.fullnames.get_default();
        let other_name = *other.fullnames.get_default();
        self_name.cmp(other_name)
    }
}

impl<'fi> PartialOrd for Font<'fi> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct SortedFamilies<'fs>(Vec<Family<'fs>>);

impl<'fs> IntoIterator for SortedFamilies<'fs> {
    type Item = <Vec<Family<'fs>> as IntoIterator>::Item;
    type IntoIter = <Vec<Family<'fs>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'fs> Deref for SortedFamilies<'fs> {
    type Target = Vec<Family<'fs>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
