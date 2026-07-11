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

use std::{fmt::Write, path::Path, sync::Arc};

use fontique::{FamilyInfo, FontInfo, SourceKind};
use skrifa::raw::{
    FontRef, TableProvider,
    tables::name::{Name, NameId, NameRecord, NameString},
    types::GlyphId,
};

use crate::{
    error::{Error, Result},
    loader::FontId,
};

/// FaceInfo contains basic font face info like family and name,
/// and pre-located glyph id for target character.
pub struct FaceInfo {
    pub id: FontId,
    pub path: Arc<Path>,
    pub index: u32,
    pub family: String,
    pub name: String,
    pub gid: GlyphId,
}

impl FaceInfo {
    pub fn parse_if_contains(
        id: FontId, f: &FamilyInfo, font: &FontInfo, rf: FontRef<'_>, c: char,
    ) -> Result<Option<Self>> {
        // check glyph exists
        let Some(glyph) = rf.cmap()?.map_codepoint(c) else {
            return Ok(None);
        };

        // get font name
        let index = font.index();
        let path = match font.source().kind() {
            SourceKind::Memory(_) => return Ok(None),
            SourceKind::Path(path) => Arc::clone(path),
        };

        let name = Self::get_full_name(rf, path.as_ref(), index)?;

        Ok(Some(FaceInfo {
            id,
            family: f.name().to_string(),
            path,
            name,
            index,
            gid: glyph,
        }))
    }

    fn record_is_english(rec: &NameRecord) -> bool {
        // Unicode
        rec.platform_id() == 0 ||
        /* Mac     */
        rec.platform_id() == 1 && rec.language_id() == 0 ||
        /* Windows */
        rec.platform_id() == 3 && (rec.language_id() == 0x0009 || rec.language_id() == 0x0409)
    }

    fn get_record<'d>(table: &Name<'d>, id: NameId) -> Result<NameString<'d>> {
        let mut result = None;
        for rec in table.name_record().iter().filter(|rec| rec.name_id() == id) {
            let is_english = Self::record_is_english(rec);

            if result.is_none() || is_english {
                if let Ok(s) = rec.string(table.string_data()) {
                    result = Some(s);
                    if is_english {
                        break;
                    }
                }
            }
        }
        return result.ok_or(Error::MissingName);
    }

    fn get_full_name(rf: FontRef<'_>, path: &Path, index: u32) -> Result<String> {
        static CANDIDATES: &[(&[NameId], &str)] = &[
            (&[NameId::FULL_NAME], ""),
            (
                &[
                    NameId::TYPOGRAPHIC_FAMILY_NAME,
                    NameId::TYPOGRAPHIC_SUBFAMILY_NAME,
                ],
                "typographic family and subfamily",
            ),
            (
                &[NameId::FAMILY_NAME, NameId::SUBFAMILY_NAME],
                "family and subfamily",
            ),
            (&[NameId::POSTSCRIPT_NAME], "postscript name"),
        ];

        let table = rf.name()?;
        let mut s = String::new();
        'outer: for (candidate, desc) in CANDIDATES {
            let parts = candidate
                .iter()
                .map(|id| Self::get_record(&table, *id))
                .collect::<Result<Vec<_>>>();

            let Ok(parts) = parts else { continue };
            s.clear();
            for part in parts {
                if !s.is_empty() {
                    s.push(' ');
                }
                let Ok(_) = write!(&mut s, "{}", part) else {
                    continue 'outer;
                };
            }

            if !desc.is_empty() {
                log::info!(
                    "font {path:?}:{index} do not have a full name record, uses {desc}: {s}"
                );
            }

            return Ok(s);
        }

        Err(Error::MissingName)
    }
}
