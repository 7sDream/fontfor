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

use std::{
    cell::{Cell, RefCell, RefMut},
    collections::hash_map::HashMap,
    rc::Rc,
};

use tui::widgets::ListState;

use super::cache::{CacheKey, GlyphCache, GlyphCanvasShape, RenderType, CHAR_RENDERS, MONO_RENDER};
use crate::{
    family::Family,
    loader::{FaceInfo, DATABASE},
    preview::terminal::{render::Render, ui::cache::GlyphParagraph},
    rasterizer::{Bitmap, Rasterizer},
};

pub struct State<'a> {
    font_faces_info: Vec<&'a FaceInfo>,
    font_faces_name: Vec<&'a str>,
    name_width_max: usize,
    list_state: RefCell<ListState>,
    height: Cell<u32>,
    width: Cell<u32>,
    rt: RenderType,
    cache: RefCell<HashMap<CacheKey, Rc<Result<GlyphCache, &'static str>>>>,
}

impl<'a> State<'a> {
    pub fn new(families: Vec<Family<'a>>) -> Self {
        let font_faces_info: Vec<_> = families
            .into_iter()
            .flat_map(|f| f.faces.into_iter())
            .collect();
        let font_faces_name: Vec<_> = font_faces_info.iter().map(|f| f.name.as_ref()).collect();
        let name_width_max = font_faces_name
            .iter()
            .map(|f| f.len())
            .max()
            .unwrap_or_default();

        let cache = RefCell::default();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            font_faces_info,
            font_faces_name,
            name_width_max,
            list_state: RefCell::new(list_state),
            height: Cell::new(0),
            width: Cell::new(0),
            rt: RenderType::Mono,
            cache,
        }
    }

    fn cache_key(&self, width: u32, height: u32) -> CacheKey {
        CacheKey {
            index: self.index(),
            rt: self.rt,
            width,
            height,
        }
    }

    pub fn render(&self) -> Rc<Result<GlyphCache, &'static str>> {
        let (width, height) = match self.rt {
            RenderType::Mono => self.get_canvas_size_by_pixel(),
            _ => self.get_canvas_size_by_char(),
        };

        let key = self.cache_key(width, height);
        self.cache
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| Rc::new(self.real_render(width, height)))
            .clone()
    }

    fn rasterize(&self, _width: u32, height: u32) -> Result<Bitmap, &'static str> {
        let info = self.font_faces_info[self.index()];

        let scale = if matches!(self.rt, RenderType::AsciiLevel10 | RenderType::AsciiLevel70) {
            Some(2.0)
        } else {
            None
        };

        DATABASE
            .with_face_data(info.id, |data, index| -> Option<Bitmap> {
                let mut r = Rasterizer::new(data, index).ok()?;
                r.set_pixel_height(height);
                if let Some(scale) = scale {
                    r.set_hscale(scale);
                }
                r.rasterize(info.gid.0)
            })
            .ok_or("Can't read this font file")?
            .ok_or("Can't get glyph from this font")
    }

    fn real_render(&self, width: u32, height: u32) -> Result<GlyphCache, &'static str> {
        let bitmap = self.rasterize(width, height)?;
        let cache = match self.rt {
            RenderType::Mono => GlyphCache::Canvas(GlyphCanvasShape::new(
                MONO_RENDER.render(&bitmap),
                width as f64,
                height as f64,
            )),
            rt => GlyphCache::Paragraph(GlyphParagraph::new(
                CHAR_RENDERS
                    .get(&rt)
                    .expect("all render must be exist")
                    .render(&bitmap),
            )),
        };

        Ok(cache)
    }

    pub fn current_name(&self) -> &'a str {
        self.font_faces_name[self.index()]
    }

    pub fn name_width_max(&self) -> usize {
        self.name_width_max
    }

    pub fn family_names(&self) -> &Vec<&'a str> {
        &self.font_faces_name
    }

    pub fn mut_list_state(&self) -> RefMut<'_, ListState> {
        self.list_state.borrow_mut()
    }

    pub fn index(&self) -> usize {
        self.list_state
            .borrow()
            .selected()
            .expect("always has a selected item")
    }

    pub fn move_up(&mut self) {
        let changed = self
            .list_state
            .borrow()
            .selected()
            .map(|index| index.saturating_sub(1));
        self.list_state.borrow_mut().select(changed);
    }

    pub fn move_down(&mut self) {
        let changed = self.list_state.borrow().selected().map(|index| {
            index
                .saturating_add(1)
                .min(self.font_faces_name.len().saturating_sub(1))
        });
        self.list_state.borrow_mut().select(changed);
    }

    pub fn get_render_type(&self) -> &RenderType {
        &self.rt
    }

    pub fn next_render_type(&mut self) {
        self.rt = match self.rt {
            RenderType::AsciiLevel10 => RenderType::AsciiLevel70,
            RenderType::AsciiLevel70 => RenderType::Moon,
            RenderType::Moon => RenderType::Mono,
            RenderType::Mono => RenderType::AsciiLevel10,
        }
    }

    pub fn prev_render_type(&mut self) {
        self.rt = match self.rt {
            RenderType::AsciiLevel10 => RenderType::Mono,
            RenderType::AsciiLevel70 => RenderType::AsciiLevel10,
            RenderType::Moon => RenderType::AsciiLevel70,
            RenderType::Mono => RenderType::Moon,
        }
    }

    pub fn update_canvas_size_by_char(&self, width: u32, height: u32) {
        self.width.replace(width);
        self.height.replace(height);
    }

    pub fn get_canvas_size_by_char(&self) -> (u32, u32) {
        (self.width.get(), self.height.get())
    }

    pub fn get_canvas_size_by_pixel(&self) -> (u32, u32) {
        (self.width.get() * 2, self.height.get() * 4)
    }
}
