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

use std::path::{Path, PathBuf};

/// This struct is a convenient type to represent fonts in `FontSet`'s font array.
pub struct FontInfo<'db> {
    pub id: fontdb::ID,
    pub path: PathBuf,
    pub index: u32,
    pub family: String,
    pub name: String,
    pub cmap: Vec<owned_ttf_parser::cmap::Subtable<'db>>,
}

impl<'font> FontInfo<'font> {
    pub fn family_name(&self) -> &str {
        &self.family
    }

    pub fn fullname(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}
