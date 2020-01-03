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
    super::fc::{FontInfo, FontSet, StrValuesByLang, ValuesByLang},
    std::{
        cmp::Ordering,
        collections::{BinaryHeap, HashMap},
        convert::TryFrom,
        slice::Iter,
    },
    unicode_width::UnicodeWidthStr,
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

impl<'a, T> GetValueByLang for ValuesByLang<'a, T> {
    type Item = T;

    fn get_by_lang(&self, lang: &str) -> Option<&Self::Item> {
        self.get(lang).and_then(|values| values.first())
    }

    fn when_missing(&self) -> &Self::Item {
        self.values().next().unwrap().first().unwrap()
    }
}

pub struct Family<'a> {
    pub name: StrValuesByLang<'a>,
    pub fonts: BinaryHeap<Font<'a>>,
    pub default_name_width: usize,
}

impl<'a> Family<'a> {
    pub fn new(name: StrValuesByLang<'a>) -> Self {
        let default_name = *name.get_default();
        let default_name_width = UnicodeWidthStr::width(default_name);
        Self { name, fonts: BinaryHeap::new(), default_name_width }
    }

    pub fn styles_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn add_font(&mut self, font: Font<'a>) -> &mut Self {
        self.fonts.push(font);
        self
    }
}

#[derive(Eq)]
pub struct Font<'a> {
    pub family_names: StrValuesByLang<'a>,
    pub fullnames: StrValuesByLang<'a>,
}

impl<'a> PartialEq for Font<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.fullnames.get_default() == other.fullnames.get_default()
    }
}

/// Implement `Ord` trait for store `FontInfo` in `BinaryHeap` struct
///
/// We sort font by it's fullname of default language(en).
impl<'a> Ord for Font<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_name = *self.fullnames.get_default();
        let other_name = *other.fullnames.get_default();
        self_name.cmp(other_name)
    }
}

impl<'a> PartialOrd for Font<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> TryFrom<FontInfo<'a>> for Font<'a> {
    type Error = ();

    fn try_from(font_info: FontInfo<'a>) -> Result<Self, Self::Error> {
        let f = Self { family_names: font_info.family_names()?, fullnames: font_info.fullnames()? };
        if f.family_names.is_empty() || f.fullnames.is_empty() {
            Err(())
        } else {
            Ok(f)
        }
    }
}

pub struct SortedFamilies<'a>(Vec<Family<'a>>);

impl<'a> From<&'a FontSet> for SortedFamilies<'a> {
    fn from(font_set: &'a FontSet) -> Self {
        let mut families: HashMap<&str, Family> = HashMap::new();

        font_set.fonts().for_each(|fc_font| {
            if let Ok(font) = Font::try_from(fc_font) {
                let family = font.family_names.get_default();
                families
                    .entry(*family)
                    .or_insert_with(|| Family::new(font.family_names.clone()))
                    .add_font(font);
            }
        });

        let mut families: Vec<Family<'a>> =
            families.into_iter().map(|(_, family)| family).collect();

        families.sort_by_key(|f| -> &'a str { f.name.get_default() });

        Self(families)
    }
}

impl<'a> IntoIterator for SortedFamilies<'a> {
    type Item = <Vec<Family<'a>> as IntoIterator>::Item;
    type IntoIter = <Vec<Family<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> SortedFamilies<'a> {
    pub fn iter(&self) -> Iter<'_, Family<'a>> {
        self.0.iter()
    }
}
