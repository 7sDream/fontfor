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

mod ascii;
mod mono;
mod moon;

use std::{
    borrow::Cow,
    fmt::{Display, Error, Formatter, Write},
};

pub use {
    ascii::{AsciiRender, AsciiRenders},
    mono::MonoRender,
    moon::MoonRender,
};

#[derive(Clone)]
pub struct RenderResult(pub Vec<Vec<char>>);

impl Display for RenderResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for line in &self.0 {
            for c in line.iter() {
                f.write_char(*c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl RenderResult {
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0.first().map_or(0, Vec::len)
    }
}

pub trait CharBitmapRender {
    #[allow(clippy::too_many_arguments)] // need them..., fine, I will try make them a struct
    fn gray_to_char(&self, up: u8, left: u8, gray: u8, right: u8, down: u8) -> char;

    fn render(&self, bitmap: &[Cow<'_, [u8]>]) -> RenderResult {
        let height = bitmap.len();
        let width = bitmap.get(0).map(|row| row.len()).unwrap_or_default();
        RenderResult(
            (0..height)
                .map(|row| {
                    (0..width)
                        .map(move |col| {
                            let gray = bitmap[row][col];

                            let l = if col > 0 { bitmap[row][col - 1] } else { 0 };
                            let r = if col < width - 1 { bitmap[row][col] } else { 0 };
                            let u = if row > 0 { bitmap[row - 1][col] } else { 0 };
                            let d = if row < height - 1 { bitmap[row + 1][col] } else { 0 };

                            self.gray_to_char(u, l, gray, r, d)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
}
