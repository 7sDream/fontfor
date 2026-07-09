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

mod face_info;

use std::{cell::RefCell, fmt::Debug, path::Path};

use fontique::{Blob, Collection, CollectionOptions, FamilyId, SourceCache, SourceCacheOptions};
use skrifa::FontRef;

pub use self::face_info::FaceInfo;
use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(FamilyId, usize);

pub struct Database {
    collection: RefCell<Collection>,
    cache: RefCell<SourceCache>,
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Database").finish()
    }
}

impl Database {
    pub fn new<I, P>(system: bool, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut collection = Collection::new(CollectionOptions {
            system_fonts: system,
            shared: false,
        });
        collection.load_fonts_from_paths(paths);

        let cache = SourceCache::new(SourceCacheOptions { shared: false });

        Self {
            collection: RefCell::new(collection),
            cache: RefCell::new(cache),
        }
    }

    fn names(&self) -> Vec<String> {
        let mut collection = self.collection.borrow_mut();
        collection.family_names().map(|s| s.to_string()).collect()
    }

    pub fn data(&self, FontId(id, index): FontId) -> Option<Blob<u8>> {
        let mut collection = self.collection.borrow_mut();
        let mut cache = self.cache.borrow_mut();

        let family = collection.family(id)?;
        let font = family.fonts().get(index)?;
        font.load(Some(&mut cache))
    }

    pub fn query(&self, c: char) -> Vec<FaceInfo> {
        let names = self.names();
        log::info!("family count: {}", names.len());
        let mut result = Vec::new();

        let mut collection = self.collection.borrow_mut();
        let mut cache = self.cache.borrow_mut();

        for name in names {
            let Some(id) = collection.family_id(&name) else {
                log::warn!("family '{}' skipped due to can't find family info", name);
                continue;
            };
            let Some(family) = collection.family(id) else {
                log::warn!("family '{}' skipped due to can't find family info", name);
                continue;
            };
            for (i, font) in family.fonts().iter().enumerate() {
                let Some(blob) = font.load(Some(&mut cache)) else {
                    log::warn!(
                        "family {} font {} skipped due to can't load font data",
                        name,
                        i
                    );
                    continue;
                };
                let rf = match FontRef::from_index(blob.data(), font.index()) {
                    Ok(rf) => rf,
                    Err(err) => {
                        log::warn!(
                            "family {} font {} skipped due to fail to parse font header of file \
                             {:?}:{}: {}",
                            name,
                            i,
                            font.source(),
                            font.index(),
                            err,
                        );
                        continue;
                    }
                };

                let font_id = FontId(id, i);

                match FaceInfo::parse_if_contains(font_id, &family, font, rf, c) {
                    Ok(Some(face)) => result.push(face),
                    Ok(None) => continue,
                    Err(err) => {
                        log::warn!(
                            "family {} font {} skipped due to fail to parse font tables of file \
                             {:?}: {}",
                            name,
                            i,
                            font.source(),
                            err
                        )
                    }
                }
            }
        }

        result
    }
}
