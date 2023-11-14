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

use ab_glyph::OutlinedGlyph;
use grid::Grid;

pub struct Metrics {
    pub left: usize,
    pub top: usize,
    pub height: usize,
    pub width: usize,
}

pub struct Bitmap {
    metrics: Metrics,
    bitmap: Grid<u8>,
}

impl Bitmap {
    pub fn new(curves: &OutlinedGlyph) -> Self {
        let bound = curves.px_bounds();

        let metrics = Metrics {
            left: bound.min.x as usize,
            top: bound.min.y as usize,
            height: bound.height() as usize,
            width: bound.width() as usize,
        };

        let mut bitmap = Grid::new(metrics.height, metrics.width);

        curves.draw(|x, y, c| {
            let value = (c * 255.0).round() as u8;
            bitmap[y as usize][x as usize] = value
        });

        Self { metrics, bitmap }
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub fn pixel(&self, row: usize, col: usize) -> u8 {
        self.bitmap[row][col]
    }
}
