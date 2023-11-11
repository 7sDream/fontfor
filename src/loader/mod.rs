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

mod charset;
mod font_info;
mod font_set;
mod pattern;

pub use charset::Charset;
pub use font_info::FontInfo;
pub use font_set::FontSet;
use once_cell::sync::Lazy;
pub use pattern::Pattern;

pub static DATABASE: Lazy<fontdb::Database,> = Lazy::new(|| {
    let mut db = fontdb::Database::default();
    db.load_system_fonts();
    return db;
},);

pub fn init() {}
