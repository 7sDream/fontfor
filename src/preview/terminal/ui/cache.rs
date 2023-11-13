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

use std::{collections::HashMap, iter::Iterator};

use grid::Grid;
use once_cell::sync::Lazy;
use tui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

use crate::preview::terminal::render::{AsciiRender, AsciiRenders, MonoRender, MoonRender, Render};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RenderType {
    AsciiLevel10,
    AsciiLevel70,
    Moon,
    Mono,
}

type BoxedRender<Pixel> = Box<dyn Render<Pixel = Pixel> + Send + Sync>;

pub static CHAR_RENDERS: Lazy<HashMap<RenderType, BoxedRender<char>>> = Lazy::new(|| {
    let mut renders: HashMap<RenderType, BoxedRender<char>> = HashMap::new();
    renders.insert(RenderType::AsciiLevel10, Box::new(AsciiRender::new(AsciiRenders::Level10)));
    renders.insert(RenderType::AsciiLevel70, Box::new(AsciiRender::new(AsciiRenders::Level70)));
    renders.insert(RenderType::Moon, Box::new(MoonRender::new()));
    renders
});

pub static MONO_RENDER: Lazy<BoxedRender<bool>> = Lazy::new(|| Box::<MonoRender>::default());

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CacheKey {
    pub index: usize,
    pub rt: RenderType,
    pub height: u32,
    pub width: u32,
}

pub enum GlyphCache {
    Canvas(GlyphCanvasShape),
    Paragraph(GlyphParagraph),
}

pub struct GlyphCanvasShape {
    h_pad: f64,
    v_pad: f64,
    canvas_height: f64,
    bitmap: Grid<bool>,
}

impl GlyphCanvasShape {
    pub fn new(bitmap: Grid<bool>, canvas_width: f64, canvas_height: f64) -> Self {
        let h_pad = ((canvas_width - bitmap.cols() as f64) / 2.0).floor();
        let v_pad = ((canvas_height - bitmap.rows() as f64) / 2.0).floor();
        Self { h_pad, v_pad, canvas_height, bitmap }
    }

    fn points(&self) -> GlyphCanvasShapePoints<'_> {
        GlyphCanvasShapePoints::new(self)
    }
}

struct GlyphCanvasShapePoints<'a> {
    shape: &'a GlyphCanvasShape,
    start: bool,
    x: usize,
    y: usize,
}

impl<'a> GlyphCanvasShapePoints<'a> {
    fn new(shape: &'a GlyphCanvasShape) -> Self {
        Self { shape, start: false, x: 0, y: 0 }
    }

    fn next_x_y(&mut self) -> bool {
        if self.start {
            self.x += 1;
            if self.x >= self.shape.bitmap.cols() {
                self.y += 1;
                self.x = 0;
            }
        } else {
            self.start = true;
        }
        self.y < self.shape.bitmap.rows()
    }
}

impl<'a> Iterator for GlyphCanvasShapePoints<'a> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.next_x_y() {
                return None;
            }
            if self.shape.bitmap[self.y][self.x] {
                // tui canvas origin point at left bottom but chars' at left top
                // so we need do some math to flip it and add padding
                let result = (
                    self.x as f64 + self.shape.h_pad,
                    self.shape.canvas_height - self.y as f64 - self.shape.v_pad,
                );
                return Some(result);
            }
        }
    }
}

impl Shape for GlyphCanvasShape {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for (x, y) in self.points() {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, Color::Reset);
            }
        }
    }
}

pub struct GlyphParagraph {
    pub lines: Vec<String>,
}

impl GlyphParagraph {
    pub fn new(bitmap: Grid<char>) -> Self {
        let lines = bitmap.iter_rows().map(String::from_iter).collect();
        Self { lines }
    }
}
