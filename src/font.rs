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
    super::fc::{FontInfo, FontSet, StrValuesByLang, ValuesByLang},
    std::{
        cmp::{Ordering, Reverse},
        collections::{BinaryHeap, HashMap},
        convert::TryFrom,
        ops::Deref,
        os::raw::c_int,
    },
};

const DEFAULT_LANG: &str = "en";

/// Convenient trait for quickly get property value in default language
pub trait GetValueByLang {
    type Item;

    fn get_by_lang(&self, lang: &str) -> Option<&Self::Item>;

    fn when_missing(&self) -> &Self::Item;

    fn get_default(&self) -> &Self::Item {
        self.get_by_lang(DEFAULT_LANG).unwrap_or_else(|| self.when_missing())
    }
}

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
    pub index: c_int,
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

impl<'fi> TryFrom<FontInfo<'fi>> for Font<'fi> {
    type Error = ();

    fn try_from(font_info: FontInfo<'fi>) -> Result<Self, Self::Error> {
        let f = Self {
            family_names: font_info.family_names()?,
            fullnames: font_info.fullnames()?,
            path: font_info.path()?,
            index: font_info.index()?,
        };
        if f.family_names.is_empty() || f.fullnames.is_empty() {
            Err(())
        } else {
            Ok(f)
        }
    }
}

pub struct SortedFamilies<'fs>(Vec<Family<'fs>>);

impl<'fs> From<&'fs FontSet> for SortedFamilies<'fs> {
    fn from(font_set: &'fs FontSet) -> Self {
        let mut families = HashMap::new();

        font_set.fonts().for_each(|fc_font| {
            if let Ok(font) = Font::try_from(fc_font) {
                let family = font.family_names.get_default();
                families
                    .entry(*family)
                    .or_insert_with(|| Family::new(font.family_names.clone()))
                    .add_font(font);
            }
        });

        let mut families: Vec<Family<'fs>> =
            families.into_iter().map(|(_, family)| family).collect();

        families.sort_by_key(|f| -> &'fs str { f.name.get_default() });

        Self(families)
    }
}

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
