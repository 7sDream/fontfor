// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2023 7sDream <i@7sdre.am> and contributors
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

use std::{borrow::Cow, path::Path};

use fontdb::Source;
use ttf_parser::{
    name::{name_id, Table as NameTable},
    Language, RawFace,
};

use super::{
    cmap::CMapTable,
    error::{BROKEN_NAME_TABLE, MISSING_NAME_TABLE, NAME_TAG},
    Error, Result,
};
use crate::loader::database;

/// FaceInfo contains basic font face info like family and name,
/// and pre-located glyph id for target character.
pub struct FaceInfo {
    pub id: fontdb::ID,

    pub family: &'static str,
    pub name: Cow<'static, str>,

    pub path: &'static Path,
    pub index: u32,

    pub gid: u16,
}

enum FontFaceFullName {
    Full(String),
    SubFamily(String),
    None,
}

impl FaceInfo {
    pub fn parse_if_contains(face: &'static fontdb::FaceInfo, c: char) -> Result<Option<Self>> {
        let path = match face.source {
            Source::File(ref path) => path,
            _ => unreachable!("we only load font file, so source must be File variant"),
        };

        let index = face.index;

        let Some((gid, name)) = database()
            .with_face_data(face.id, |data, index| -> Result<_> {
                let rf = RawFace::parse(data, index)?;
                let Some(gid) = CMapTable::parse(rf)?.glyph_index(c) else {
                    return Ok(None);
                };

                let name = Self::parse_full_name(rf)?;
                Ok(Some((gid.0, name)))
            })
            .expect("we only load font from database so it must not None")?
        else {
            return Ok(None);
        };

        let family = face
            .families
            .first()
            .map(|(s, _)| s.as_str())
            .ok_or(Error::MissingFamilyName)?;

        let name: Cow<'static, str> = match name {
            FontFaceFullName::Full(full) => full.into(),
            FontFaceFullName::SubFamily(sub) => {
                log::info!(
                    "Font face {}:{} do not have a full name, uses family({}) + subfamily({})",
                    path.to_string_lossy(),
                    index,
                    family,
                    sub,
                );

                if sub.is_empty() {
                    Cow::Borrowed(family)
                } else {
                    format!("{} {}", family, sub).into()
                }
            }
            FontFaceFullName::None => {
                log::info!(
                    "Font face {}:{} do not have a full name and subfamily, uses postscript \
                     name({})",
                    path.to_string_lossy(),
                    index,
                    face.post_script_name,
                );

                Cow::Borrowed(&face.post_script_name)
            }
        };

        Ok(Some(FaceInfo {
            id: face.id,
            family,
            name,
            path,
            index: face.index,
            gid,
        }))
    }

    fn parse_full_name(rf: RawFace<'_>) -> Result<FontFaceFullName> {
        let name_data = rf.table(NAME_TAG).ok_or(MISSING_NAME_TABLE)?;
        let name_table = NameTable::parse(name_data).ok_or(BROKEN_NAME_TABLE)?;

        let mut sub_family: Option<String> = None;

        for i in 0..name_table.names.len() {
            let name = name_table.names.get(i).ok_or(BROKEN_NAME_TABLE)?;

            if name.language() == Language::English_UnitedStates && name.is_unicode() {
                if name.name_id == name_id::FULL_NAME {
                    if let Some(name) = name.to_string() {
                        return Ok(FontFaceFullName::Full(name));
                    }
                }
                if name.name_id == name_id::SUBFAMILY {
                    if let Some(name) = name.to_string() {
                        sub_family.replace(name);
                    }
                }
            }
        }

        if let Some(sub) = sub_family {
            Ok(FontFaceFullName::SubFamily(sub))
        } else {
            Ok(FontFaceFullName::None)
        }
    }
}
