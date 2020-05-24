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
mod fc; // means FontConfig
#[cfg(target_os = "linux")]
pub use fc::FontSet;

#[cfg(target_os = "macos")]
mod ct; // means Core Text api
#[cfg(target_os = "macos")]
pub use ct::FontSet;

use {super::Font, std::convert::TryInto};

pub trait FontMatcher<'fs, T>
where
    T: TryInto<Font<'fs>>,
{
    type Output: Iterator<Item = T>;

    fn init() -> Result<(), ()>;
    fn finalize();
    fn fonts_contains(c: char) -> Self;
    fn fonts(&'fs self) -> Self::Output;
}
