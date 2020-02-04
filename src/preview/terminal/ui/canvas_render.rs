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

use {
    super::super::render::RenderResult,
    std::iter::Iterator,
    tui::{style::Color, widgets::canvas::Shape},
};

pub struct CanvasRenderResult<'a> {
    chars: &'a RenderResult,
    canvas_width: f64,
    canvas_height: f64,
}

impl<'a> CanvasRenderResult<'a> {
    pub const fn new(r: &'a RenderResult, canvas_width: f64, canvas_height: f64) -> Self {
        Self { chars: r, canvas_width, canvas_height }
    }
}

struct RenderResultPoints {
    start: bool,
    x: usize,
    y: usize,
    height: f64,
    h_pad: f64,
    v_pad: f64,
    chars: RenderResult,
}

impl RenderResultPoints {
    #[allow(clippy::cast_precision_loss)] // render result size is small enough to cast to f64
    fn new(chars: &RenderResult, width: f64, height: f64) -> Self {
        let h_pad = ((width - chars.width() as f64) / 2.0).floor();
        let v_pad = ((height - chars.height() as f64) / 2.0).floor();
        Self { start: false, x: 0, y: 0, height, h_pad, v_pad, chars: chars.clone() }
    }

    fn next_x_y(&mut self) -> bool {
        if self.start {
            self.x += 1;
            if self.x >= self.chars.width() {
                self.y += 1;
                self.x = 0;
            }
        } else {
            self.start = true;
        }
        self.y < self.chars.0.len()
    }
}

impl Iterator for RenderResultPoints {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.next_x_y() {
                return None;
            }
            if self.chars.0[self.y][self.x] != ' ' {
                // tui canvas origin point at left bottom but chars' at left top
                // so we need do some math to flip it and add padding
                #[allow(clippy::cast_precision_loss)] // render result size is small enough
                let result = (self.x as f64 + self.h_pad, self.height - self.y as f64 - self.v_pad);
                return Some(result);
            }
        }
    }
}

impl<'a> Shape<'a> for CanvasRenderResult<'a> {
    fn color(&self) -> Color {
        Color::Reset
    }

    fn points(&'a self) -> Box<dyn Iterator<Item = (f64, f64)>> {
        Box::new(RenderResultPoints::new(self.chars, self.canvas_width, self.canvas_height))
    }
}
