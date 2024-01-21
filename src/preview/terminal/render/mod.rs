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

mod ascii;
mod mono;
mod moon;

use grid::Grid;

pub use self::{
    ascii::{AsciiRender, AsciiRenders},
    mono::MonoRender,
    moon::MoonRender,
};
use crate::rasterizer::Bitmap;

pub trait Render {
    type Pixel: Default;

    #[allow(clippy::too_many_arguments)] // need them..., fine, I will try make them a struct
    fn render_pixel(&self, up: u8, left: u8, gray: u8, right: u8, down: u8) -> Self::Pixel;

    fn render(&self, bm: &Bitmap) -> Grid<Self::Pixel> {
        let m = bm.metrics();

        let mut result = Grid::new(m.height, m.width);

        for row in 0..m.height {
            for col in 0..m.width {
                let gray = bm.pixel(row, col);

                let l = if col > 0 { bm.pixel(row, col - 1) } else { 0 };
                let r = if col < m.width - 1 {
                    bm.pixel(row, col + 1)
                } else {
                    0
                };
                let u = if row > 0 { bm.pixel(row - 1, col) } else { 0 };
                let d = if row < m.height - 1 {
                    bm.pixel(row + 1, col)
                } else {
                    0
                };

                result[(row, col)] = self.render_pixel(u, l, gray, r, d)
            }
        }

        result
    }
}
