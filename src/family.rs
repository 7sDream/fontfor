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

use std::collections::HashMap;

use super::loader::FaceInfo;

pub struct Family<'a> {
    pub name: &'a str,
    pub faces: Vec<&'a FaceInfo>,
    pub default_name_width: usize,
}

impl<'a> Family<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, faces: vec![], default_name_width: name.len() }
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
        families.entry(family).or_insert_with(|| Family::new(family)).add_face(face);
    });

    let mut families: Vec<Family<'_>> = families.into_values().collect();

    families.sort_by_key(|f| f.name);

    for family in &mut families {
        family.faces.sort_unstable_by(|a, b| a.name.cmp(&b.name))
    }

    families
}
