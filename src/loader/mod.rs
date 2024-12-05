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

use std::path::Path;

use fontdb::Database;
use once_cell::sync::OnceCell;

pub use self::{error::Error, face_info::FaceInfo};
pub type Result<T> = std::result::Result<T, Error>;

static DATABASE: OnceCell<Database> = OnceCell::new();

pub fn init<I, P>(system: bool, paths: I)
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut db = Database::default();

    if system {
        db.load_system_fonts();
    }

    for path in paths.into_iter() {
        db.load_fonts_dir(path)
    }

    DATABASE.set(db).expect("call init only once")
}

pub fn database() -> &'static Database {
    DATABASE.get().expect("use after init")
}

pub fn query(c: char) -> Vec<FaceInfo> {
    database()
        .faces()
        .filter_map(|info| {
            let face = FaceInfo::parse_if_contains(info, c);

            if let Err(ref err) = face {
                log::warn!("Fail to get font face name of {:?}: {}", info.source, err)
            }

            face.transpose()
        })
        .filter_map(|f| f.ok())
        .collect()
}
