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

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    convert::TryFrom,
    ops::Deref,
};

use super::loader::{FontInfo, FontSet};
use crate::loader;

pub struct Family<'a, 'db,> {
    pub name: &'a str,
    pub fonts: BinaryHeap<Reverse<Font<'a, 'db,>,>,>,
    pub default_name_width: usize,
}

impl<'a, 'db: 'a,> Family<'a, 'db,> {
    pub fn new(name: &'a str,) -> Self {
        Self { name, fonts: BinaryHeap::new(), default_name_width: name.len(), }
    }

    pub fn styles_count(&self,) -> usize {
        self.fonts.len()
    }

    pub fn add_font(&mut self, font: Font<'a, 'db,>,) -> &mut Self {
        self.fonts.push(Reverse(font,),);
        self
    }
}

pub struct Font<'a, 'db,>(pub &'a loader::FontInfo<'db,>,);

impl<'a, 'db,> PartialEq for Font<'a, 'db,> {
    fn eq(&self, other: &Self,) -> bool {
        self.0.fullname() == other.0.fullname()
    }
}

impl<'a, 'db,> Eq for Font<'a, 'db,> {}

/// Implement `Ord` trait for store `FontInfo` in `BinaryHeap` struct
impl<'a, 'db,> Ord for Font<'a, 'db,> {
    fn cmp(&self, other: &Self,) -> Ordering {
        self.0.fullname().cmp(other.0.fullname(),)
    }
}

impl<'a, 'db,> PartialOrd for Font<'a, 'db,> {
    fn partial_cmp(&self, other: &Self,) -> Option<Ordering,> {
        Some(self.cmp(other,),)
    }
}

impl<'a, 'db,> TryFrom<&'a FontInfo<'db,>,> for Font<'a, 'db,> {
    type Error = ();

    fn try_from(font_info: &'a FontInfo<'db,>,) -> Result<Self, Self::Error,> {
        Ok(Self(font_info,),)
    }
}

pub struct SortedFamilies<'a, 'db,>(Vec<Family<'a, 'db,>,>,);

impl<'a, 'db: 'a,> From<&'a loader::FontSet<'db,>,> for SortedFamilies<'a, 'db,> {
    fn from(font_set: &'a FontSet<'db,>,) -> Self {
        let mut families = HashMap::new();

        font_set.fonts().iter().for_each(|fc_font| {
            if let Ok(font,) = Font::try_from(fc_font,) {
                let family = font.0.family_name();
                families.entry(family,).or_insert_with(|| Family::new(family,),).add_font(font,);
            }
        },);

        let mut families: Vec<Family<'a, 'db,>,> =
            families.into_iter().map(|(_, family,)| family,).collect();

        families.sort_by_key(|f| f.name,);

        Self(families,)
    }
}

impl<'a, 'db,> IntoIterator for SortedFamilies<'a, 'db,> {
    type IntoIter = <Vec<Family<'a, 'db,>,> as IntoIterator>::IntoIter;
    type Item = <Vec<Family<'a, 'db,>,> as IntoIterator>::Item;

    fn into_iter(self,) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, 'db,> Deref for SortedFamilies<'a, 'db,> {
    type Target = Vec<Family<'a, 'db,>,>;

    fn deref(&self,) -> &Self::Target {
        &self.0
    }
}
