// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2025 7sDream <i@7sdre.am> and contributors
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

use ab_glyph_rasterizer::Rasterizer;
use grid::Grid;

pub struct Metrics {
    pub width: usize,
    pub height: usize,
}

pub struct Bitmap {
    metrics: Metrics,
    bitmap: Grid<u8>,
}

impl Bitmap {
    pub fn new(r: Rasterizer) -> Self {
        let (width, height) = r.dimensions();
        let metrics = Metrics { width, height };

        let mut bitmap = Grid::new(metrics.height, metrics.width);

        r.for_each_pixel_2d(|x, y, c| {
            let value = (c * 255.0).round() as u8;
            bitmap[(y as usize, x as usize)] = value
        });

        Self { metrics, bitmap }
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub fn pixel(&self, row: usize, col: usize) -> u8 {
        self.bitmap[(row, col)]
    }
}
