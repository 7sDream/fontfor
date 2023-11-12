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

use owned_ttf_parser::{cmap, GlyphId, RawFace};

use super::{
    error::{BROKEN_CMAP_TABLE, CMAP_TAG, MISSING_CMAP_TABLE},
    Result,
};

#[derive(Debug)]
pub struct CMapTable<'a> {
    subtables: Vec<cmap::Subtable<'a>>,
}

impl<'a> CMapTable<'a> {
    pub fn parse(rf: RawFace<'a>) -> Result<Self> {
        let cmap_data = rf.table(CMAP_TAG).ok_or(MISSING_CMAP_TABLE)?;
        let table = cmap::Table::parse(cmap_data).ok_or(BROKEN_CMAP_TABLE)?;

        let mut subtables = vec![];

        for i in 0..table.subtables.len() {
            let subtable = table.subtables.get(i).ok_or(BROKEN_CMAP_TABLE)?;
            if subtable.is_unicode() {
                subtables.push(subtable)
            }
        }

        Ok(Self { subtables })
    }

    pub fn glyph_index(&self, c: char) -> Option<GlyphId> {
        self.subtables.iter().filter_map(|subtable| subtable.glyph_index(c as u32)).next()
    }
}
