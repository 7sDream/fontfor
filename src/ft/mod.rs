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

mod bitmap;
mod font_face;
mod library;

use freetype::freetype as ft;

pub trait CheckFreeTypeError<T> {
    fn get_err(&self) -> Option<ft::FT_Error>;
    fn as_result(&self, result: T) -> Result<T, ft::FT_Error> {
        if let Some(err) = self.get_err() {
            Err(err)
        } else {
            Ok(result)
        }
    }
    fn map_result<F>(&self, f: F) -> Result<T, ft::FT_Error>
    where
        Self: Sized,
        F: FnOnce() -> T,
    {
        if let Some(err) = self.get_err() {
            Err(err)
        } else {
            Ok(f())
        }
    }
}

impl<T> CheckFreeTypeError<T> for ft::FT_Error {
    fn get_err(&self) -> Option<i32> {
        if *self == 0 {
            None
        } else {
            Some(*self)
        }
    }
}

pub use {font_face::FontFace, library::Library};
