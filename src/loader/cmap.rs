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

use ttf_parser::{GlyphId, RawFace, cmap};

use super::{
    Result,
    error::{BROKEN_CMAP_TABLE, CMAP_TAG, MISSING_CMAP_TABLE},
};

pub struct CMapTable<'a> {
    sub_tables: Vec<cmap::Subtable<'a>>,
}

impl<'a> CMapTable<'a> {
    pub fn parse(rf: RawFace<'a>) -> Result<Self> {
        let cmap_data = rf.table(CMAP_TAG).ok_or(MISSING_CMAP_TABLE)?;
        let table = cmap::Table::parse(cmap_data).ok_or(BROKEN_CMAP_TABLE)?;

        let mut sub_tables = vec![];

        for i in 0..table.subtables.len() {
            let sub_table = table.subtables.get(i).ok_or(BROKEN_CMAP_TABLE)?;
            if sub_table.is_unicode() {
                sub_tables.push(sub_table)
            }
        }

        Ok(Self { sub_tables })
    }

    pub fn glyph_index(&self, c: char) -> Option<GlyphId> {
        self.sub_tables
            .iter()
            .filter_map(|sub_table| sub_table.glyph_index(c as u32))
            .next()
    }
}
