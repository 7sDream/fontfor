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

use std::{path::Path, sync::Arc};

use fontique::{FamilyInfo, FontInfo, SourceKind};
use skrifa::raw::{
    FontRef, TableProvider,
    tables::name::{NameId, NameRecord, NameString},
    types::GlyphId,
};

use super::{Error, Result};
use crate::loader::FontId;

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
        let (kind, name) = Self::parse_full_name(rf)?;

        let family = f.name().to_string();
        let index = font.index();
        let path = match font.source().kind() {
            SourceKind::Memory(_) => return Ok(None),
            SourceKind::Path(path) => Arc::clone(path),
        };

        // build final font name
        let name = match kind {
            NameId::FULL_NAME => name.to_string(),
            NameId::SUBFAMILY_NAME => {
                log::debug!(
                    "Font face {path:?}:{index} do not have a full name, uses family({family}) + \
                     subfamily({name})"
                );
                format!("{} {}", f.name(), name)
            }
            NameId::POSTSCRIPT_NAME => {
                log::debug!(
                    "Font face {path:?}:{index} do not have a full name and subfamily, uses \
                     postscript name({name})"
                );
                name.to_string()
            }
            _ => {
                unreachable!()
            }
        };

        Ok(Some(FaceInfo {
            id,
            family,
            path,
            name,
            index,
            gid: glyph,
        }))
    }

    fn parse_full_name(rf: FontRef) -> Result<(NameId, NameString)> {
        let is_english = |rec: &NameRecord| -> bool {
            // Unicode
            rec.platform_id() == 0 ||
            /* Mac     */
            rec.platform_id() == 1 && rec.language_id() == 0 ||
            /* Windows */
            rec.platform_id() == 3 && (rec.language_id() == 0x0009 || rec.language_id() == 0x0409)
        };

        let table = rf.name()?;
        let records = table.name_record().iter().copied().filter(is_english);

        let mut sub_family = None;
        let mut postscript = None;
        for rec in records {
            if rec.name_id() == NameId::FULL_NAME
                && let Ok(name) = rec.string(table.string_data())
            {
                return Ok((NameId::FULL_NAME, name));
            }

            if rec.name_id() == NameId::SUBFAMILY_NAME
                && let Ok(name) = rec.string(table.string_data())
            {
                sub_family.replace((NameId::SUBFAMILY_NAME, name));
            }

            if rec.name_id() == NameId::POSTSCRIPT_NAME
                && let Ok(name) = rec.string(table.string_data())
            {
                postscript.replace((NameId::POSTSCRIPT_NAME, name));
            }
        }

        if let Some(sub) = sub_family {
            Ok(sub)
        } else if let Some(post) = postscript {
            Ok(post)
        } else {
            Err(Error::MissingFamilyName)
        }
    }
}
