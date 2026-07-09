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

use ab_glyph_rasterizer::{Point, Rasterizer};
use skrifa::{outline::OutlinePen, raw::types::BoundingBox};

use super::Bitmap;

pub struct Pen {
    r: Rasterizer,
    hscale: f32,
    bound: BoundingBox<f32>,
}

impl Pen {
    pub fn new(bound: BoundingBox<f32>, hscale: f32) -> Self {
        let width = ((bound.x_max - bound.x_min) * hscale).ceil() as usize;
        let height = (bound.y_max - bound.y_min).ceil() as usize;
        let r = Rasterizer::new(width, height);

        Self { r, hscale, bound }
    }

    fn transform(&self, p: Point) -> Point {
        Point {
            x: (p.x - self.bound.x_min) * self.hscale,
            y: (self.bound.y_max - self.bound.y_min) - (p.y - self.bound.y_min),
        }
    }

    pub fn adapter(&mut self) -> PenAdapter<'_> {
        PenAdapter {
            pen: self,
            start: None,
            last: Point::default(),
        }
    }

    pub fn bitmap(self) -> Bitmap {
        Bitmap::new(self.r)
    }
}

pub struct PenAdapter<'a> {
    pen: &'a mut Pen,
    start: Option<Point>,
    last: Point,
}

impl OutlinePen for PenAdapter<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.last = self.pen.transform(Point { x, y });
        self.start = Some(self.last);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let p = self.pen.transform(Point { x, y });
        self.pen.r.draw_line(self.last, p);
        self.last = p;
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        let pc0 = self.pen.transform(Point { x: cx0, y: cy0 });
        let p = self.pen.transform(Point { x, y });
        self.pen.r.draw_quad(self.last, pc0, p);
        self.last = p;
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let pc0 = self.pen.transform(Point { x: cx0, y: cy0 });
        let pc1 = self.pen.transform(Point { x: cx1, y: cy1 });
        let p = self.pen.transform(Point { x, y });

        self.pen.r.draw_cubic(self.last, pc0, pc1, p);
        self.last = p;
    }

    fn close(&mut self) {
        if let Some(start) = self.start.take() {
            self.pen.r.draw_line(self.last, start);
            self.last = start;
        }
    }
}
