// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 7sDream <i@7sdre.am> and contributors
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
mod consts;
mod font;
mod font_set;
mod object_set;
mod pattern;

pub use {
    charset::Charset,
    font::{Font, StrByLang},
    font_set::{FontSet, Fonts},
    object_set::ObjectSet,
    pattern::Pattern,
};

use fontconfig::fontconfig as fc;

pub fn init() -> Result<(), ()> {
    let config = unsafe { fc::FcInitLoadConfigAndFonts() };
    if config.is_null() {
        Err(())
    } else {
        unsafe { fc::FcConfigDestroy(config) };
        Ok(())
    }
}

pub fn finalize() {
    unsafe {
        fc::FcFini();
    }
}
