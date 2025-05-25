// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2025 7sDream <i@7sdre.am> and contributors
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

use std::collections::HashMap;

use range_set_blaze::RangeSetBlaze;

use super::loader::FaceInfo;

pub struct Family<'a> {
    pub name: &'a str,
    pub faces: Vec<&'a FaceInfo>,
    pub default_name_width: usize,
}

impl<'a> Family<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            faces: vec![],
            default_name_width: name.len(),
        }
    }

    pub fn styles_count(&self) -> usize {
        self.faces.len()
    }

    pub fn add_face(&mut self, face: &'a FaceInfo) {
        self.faces.push(face);
    }
}

pub fn group_by_family_sort_by_name(faces: &[FaceInfo]) -> Vec<Family<'_>> {
    let mut families = HashMap::new();

    faces.iter().for_each(|face| {
        let family = &face.family;
        families
            .entry(family)
            .or_insert_with(|| Family::new(family))
            .add_face(face);
    });

    let mut families: Vec<Family<'_>> = families.into_values().collect();

    families.sort_by_key(|f| f.name);

    for family in &mut families {
        family.faces.sort_unstable_by(|a, b| a.name.cmp(&b.name))
    }

    families
}

pub struct FilteredFamilies<'a> {
    data: Vec<Family<'a>>,
    names: Vec<String>,
    keyword: String,
    filtered: RangeSetBlaze<usize>,
}

#[derive(Clone)]
pub struct FilteredFamiliesIter<'f, 'a> {
    data: &'f [Family<'a>],
    range: range_set_blaze::Iter<usize, range_set_blaze::RangesIter<'f, usize>>,
}

impl<'f, 'a: 'f> FilteredFamiliesIter<'f, 'a> {
    pub fn with_index(self) -> FilteredFamiliesWithIndexIter<'f, 'a> {
        FilteredFamiliesWithIndexIter(self)
    }
}

impl<'f, 'a: 'f> Iterator for FilteredFamiliesIter<'f, 'a> {
    type Item = &'f Family<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|i| &self.data[i])
    }
}

pub struct FilteredFamiliesWithIndexIter<'f, 'a>(FilteredFamiliesIter<'f, 'a>);

impl<'f, 'a: 'f> Iterator for FilteredFamiliesWithIndexIter<'f, 'a> {
    type Item = (usize, &'f Family<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.range.next().map(|i| (i, &self.0.data[i]))
    }
}

impl<'a> FilteredFamilies<'a> {
    pub fn new(families: Vec<Family<'a>>, keyword: String) -> Self {
        let names = families.iter().map(|f| f.name.to_lowercase()).collect();
        let mut ret = Self {
            data: families,
            names,
            keyword: keyword.to_lowercase(),
            filtered: RangeSetBlaze::new(),
        };
        ret.filter(true);
        ret
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn full_indices(&self) -> RangeSetBlaze<usize> {
        if !self.data.is_empty() {
            RangeSetBlaze::from_iter(&[0..=self.data.len() - 1])
        } else {
            RangeSetBlaze::new()
        }
    }

    pub fn matched_indices(&self) -> &RangeSetBlaze<usize> {
        &self.filtered
    }

    pub fn data(&self) -> &[Family<'a>] {
        &self.data
    }

    pub fn matched(&self) -> FilteredFamiliesIter<'_, 'a> {
        FilteredFamiliesIter {
            data: &self.data,
            range: self.matched_indices().iter(),
        }
    }

    fn unmatched_indices(&self) -> RangeSetBlaze<usize> {
        &self.full_indices() - self.matched_indices()
    }

    fn retain(rs: &mut RangeSetBlaze<usize>, data: &[String], keyword: &str) {
        if !keyword.is_empty() {
            rs.retain(|i| data[*i].contains(keyword))
        }
    }

    fn filter(&mut self, reset: bool) {
        if reset {
            self.filtered = self.full_indices()
        }

        Self::retain(&mut self.filtered, &self.names, &self.keyword)
    }

    pub fn keyword(&self) -> &str {
        &self.keyword
    }

    pub fn change_keyword(&mut self, keyword: &str) {
        let keyword = keyword.to_lowercase();

        if self.keyword == keyword {
            return;
        }

        if self.keyword.starts_with(&keyword) || self.keyword.ends_with(&keyword) {
            // more loose, only search unmatched and append to matches
            let mut unmatched = self.unmatched_indices();
            self.keyword = keyword;
            Self::retain(&mut unmatched, &self.names, &self.keyword);
            self.filtered.append(&mut unmatched)
        } else if keyword.starts_with(&keyword) || keyword.ends_with(&keyword) {
            // more strict, search currents matches again
            self.keyword = keyword;
            self.filter(false);
        } else {
            // research all
            self.keyword = keyword;
            self.filter(true)
        }
    }
}
