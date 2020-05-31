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
pub mod matcher;
pub mod render;

use {
    matcher::{FontMatcher, FontSet},
    std::{
        borrow::Cow,
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
        convert::TryFrom,
        ops::Deref,
    },
};

// TODO: Delete me if we change to use localized attribute when use Core Text API
#[cfg_attr(target_os = "macos", allow(dead_code))]
const DEFAULT_LANG: &str = "en";

pub struct Family<'fs> {
    pub name: Cow<'fs, str>,
    pub fonts: BinaryHeap<Reverse<Font<'fs>>>,
    pub name_width: usize,
}

impl<'fs> Family<'fs> {
    pub fn new(name: Cow<'fs, str>) -> Self {
        let name_width = name.len();
        Self { name, fonts: BinaryHeap::new(), name_width }
    }

    pub fn styles_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn add_font(&mut self, font: Font<'fs>) -> &mut Self {
        self.fonts.push(Reverse(font));
        self
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Font<'fi> {
    pub family_name: Cow<'fi, str>,
    pub fullname: Cow<'fi, str>,
    pub path: Cow<'fi, str>,
    pub index: usize,
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

impl<'fs> From<&'fs FontSet> for SortedFamilies<'fs> {
    fn from(font_set: &'fs FontSet) -> Self {
        let mut families: HashMap<Cow<'fs, str>, Family<'fs>> = HashMap::new();

        font_set.fonts().for_each(|fc_font| {
            if let Ok(font) = Font::try_from(fc_font) {
                if let Some(slot) = families.get_mut(&font.family_name) {
                    slot.add_font(font);
                } else {
                    let mut family = Family::new(font.family_name.clone());
                    let key = font.family_name.clone();
                    family.add_font(font);
                    families.insert(key, family);
                }
            }
        });

        let mut families: Vec<Family<'fs>> =
            families.into_iter().map(|(_, family)| family).collect();

        families.sort_by(|a, b| a.name.cmp(&b.name));

        Self(families)
    }
}
