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

use owned_ttf_parser::{
    name::{name_id, Table as NameTable},
    GlyphId, Language, RawFace,
};

use super::{
    error::{BROKEN_NAME_TABLE, MISSING_NAME_TABLE, NAME_TAG},
    Error, Result,
};
use crate::loader::{CMapTable, DATABASE};

/// FaceInfo contains basic font face info like family and name,
/// and pre-parsed cmap tables we need to query if it contains a codepoint.
pub struct FaceInfo {
    pub id: fontdb::ID,

    pub family: String,
    pub name: String,

    pub path: PathBuf,
    pub index: u32,

    pub gid: GlyphId,
}

impl FaceInfo {
    pub fn parse_if_contains(face: &fontdb::FaceInfo, c: char) -> Result<Option<Self>> {
        let Some((gid, name)) = DATABASE
            .with_face_data(face.id, |data, index| -> Result<_> {
                let rf = RawFace::parse(data, index)?;
                let Some(gid) = CMapTable::parse(rf)?.glyph_index(c) else {
                    return Ok(None);
                };

                let name = Self::parse_full_name(rf)?;
                Ok(Some((gid, name)))
            })
            .expect("we only load font from database so it must not None")?
        else {
            return Ok(None);
        };

        let family =
            face.families.get(0).map(|(s, _)| s.clone()).ok_or(Error::MissingFamilyName)?;

        let name = name.unwrap_or_else(|| face.post_script_name.clone());

        let path = match face.source {
            fontdb::Source::File(ref path) => path.clone(),
            _ => unreachable!("we only load font file, so source must be File variant"),
        };

        Ok(Some(FaceInfo { id: face.id, family, name, path, index: face.index, gid }))
    }

    fn parse_full_name(rf: RawFace<'_>) -> Result<Option<String>> {
        let name_data = rf.table(NAME_TAG).ok_or(MISSING_NAME_TABLE)?;
        let name_table = NameTable::parse(name_data).ok_or(BROKEN_NAME_TABLE)?;

        for i in 0..name_table.names.len() {
            let name = name_table.names.get(i).ok_or(BROKEN_NAME_TABLE)?;
            if name.name_id == name_id::FULL_NAME
                && name.is_unicode()
                && name.language() == Language::English_UnitedStates
            {
                if let Some(name) = name.to_string() {
                    return Ok(Some(name));
                }
            }
        }

        Ok(None)
    }
}
