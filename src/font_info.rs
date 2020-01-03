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
    super::fc::{Font, StrByLang, ValueByLang},
    std::{cmp::Ordering, collections::BinaryHeap, convert::TryFrom},
    unicode_width::UnicodeWidthStr,
};

const DEFAULT_LANG: &str = "en";

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

impl<'a, T> GetValueByLang for ValueByLang<'a, T> {
    type Item = T;

    fn get_by_lang(&self, lang: &str) -> Option<&Self::Item> {
        self.get(lang).and_then(|values| values.iter().next())
    }

    fn when_missing(&self) -> &Self::Item {
        self.values().next().unwrap().iter().next().unwrap()
    }
}

pub struct Family<'a> {
    pub name: StrByLang<'a>,
    pub fonts: BinaryHeap<FontInfo<'a>>,
    pub default_name_width: usize,
}

impl<'a> Family<'a> {
    pub fn new(name: StrByLang<'a>) -> Self {
        let default_name = *name.get_default();
        let default_name_width = UnicodeWidthStr::width(default_name);
        Self { name, fonts: BinaryHeap::new(), default_name_width }
    }

    pub fn styles_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn add_font(&mut self, font: FontInfo<'a>) -> &mut Self {
        self.fonts.push(font);
        self
    }
}

#[derive(Eq)]
pub struct FontInfo<'a> {
    pub family_names: StrByLang<'a>,
    pub fullnames: StrByLang<'a>,
}

impl<'a> PartialEq for FontInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.fullnames.get_default() == other.fullnames.get_default()
    }
}

impl<'a> Ord for FontInfo<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_name = *self.fullnames.get_default();
        let other_name = *other.fullnames.get_default();
        let length_ordering = self_name.len().cmp(&other_name.len());
        match length_ordering {
            Ordering::Equal => self_name.cmp(other_name),
            other => other,
        }
    }
}

impl<'a> PartialOrd for FontInfo<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> TryFrom<Font<'a>> for FontInfo<'a> {
    type Error = ();

    fn try_from(font: Font<'a>) -> Result<Self, Self::Error> {
        let f = Self { family_names: font.family_names()?, fullnames: font.fullnames()? };
        if f.family_names.is_empty() || f.fullnames.is_empty() {
            Err(())
        } else {
            Ok(f)
        }
    }
}
