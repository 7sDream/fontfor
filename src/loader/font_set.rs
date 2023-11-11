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

use std::path::PathBuf;

use super::{FontInfo, Pattern};
use crate::loader::DATABASE;

pub struct FontSet<'db,> {
    fonts: Vec<FontInfo<'db,>,>,
}

impl<'db,> FontSet<'db,> {
    pub fn match_pattern(pattern: &Pattern,) -> Self {
        Self {
            fonts: DATABASE
                .faces()
                .flat_map(|f| {
                    Some(FontInfo {
                        id: f.id,
                        path: PathBuf::default(),
                        index: f.index,
                        family: f.families.get(0,).map(|(s, _,)| s.clone(),)?,
                        name: f.post_script_name.clone(),
                        cmap: vec![],
                    },)
                },)
                .collect(),
        }
    }

    pub fn fonts<'fs,>(&self,) -> &[FontInfo<'db,>] {
        self.fonts.as_slice()
    }
}
