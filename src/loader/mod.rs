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

mod face_info;
mod cmap;
mod error;

use once_cell::sync::Lazy;

pub use self::{error::Error, face_info::FaceInfo};
pub type Result<T> = std::result::Result<T, Error>;

pub static DATABASE: Lazy<fontdb::Database> = Lazy::new(|| {
    let mut db = fontdb::Database::default();
    db.load_system_fonts();
    db
});

pub fn faces_contains(c: char) -> Vec<FaceInfo> {
    DATABASE
        .faces()
        .filter_map(|info| {
            let face = FaceInfo::parse_if_contains(info, c);

            if cfg!(debug_assertions) {
                if let Err(ref err) = face {
                    eprintln!("Parse {:?}: {}", info.source, err)
                }
            }

            face.transpose()
        })
        .filter_map(|f| f.ok())
        .collect()
}
