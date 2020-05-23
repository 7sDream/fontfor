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

#[cfg(target_os = "linux")]
pub mod fc; // means FontConfig
#[cfg(target_os = "linux")]
pub use fc::FontSet;

#[cfg(target_os = "macos")]
pub mod ct; // means Core Text api
#[cfg(target_os = "macos")]
pub use ct::FontSet;

pub trait FontMatcherLibrary {
    type Output;

    fn init() -> Result<(), ()>;
    fn finalize();

    fn fonts_contains(c: char) -> Self::Output;
}

#[cfg(target_os = "linux")]
impl FontMatcherLibrary for FontSet {
    type Output = Self;

    fn init() -> Result<(), ()> {
        fc::init()
    }

    fn finalize() {
        fc::finalize();
    }

    fn fonts_contains(c: char) -> Self::Output {
        let charset = fc::Charset::default().add_char(c);
        let pattern = fc::Pattern::default().add_charset(&charset);
        Self::match_pattern(&pattern)
    }
}

#[cfg(target_os = "macos")]
impl FontMatcherLibrary for FontSet {
    type Output = Self;

    fn init() -> Result<(), ()> {
        Ok(())
    }

    fn finalize() {}

    fn fonts_contains(_: char) -> Self::Output {
        Self {}
    }
}
