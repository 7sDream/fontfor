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

use ratatui::widgets::ListState;

use super::cache::{CacheKey, GlyphCache, GlyphCanvasShape, RenderType, CHAR_RENDERS, MONO_RENDER};
use crate::{
    family::FilteredFamilies,
    loader::{self, FaceInfo},
    preview::terminal::{render::Render, ui::cache::GlyphParagraph},
    rasterizer::{Bitmap, Rasterizer},
};

pub struct State<'a> {
    filtered: FilteredFamilies<'a>,
    index_map: Vec<(usize, usize)>,
    name_width_max: usize,
    list_state: RefCell<ListState>,
    height: Cell<u32>,
    width: Cell<u32>,
    rt: RenderType,
    cache: RefCell<HashMap<CacheKey, Rc<Result<GlyphCache, &'static str>>>>,
}

impl<'a> State<'a> {
    pub fn new(filtered: FilteredFamilies<'a>) -> Self {
        let mut ret = Self {
            filtered,
            index_map: Vec::new(),
            name_width_max: 0,
            list_state: RefCell::default(),
            height: Cell::new(0),
            width: Cell::new(0),
            rt: RenderType::Mono,
            cache: RefCell::default(),
        };

        ret.update_search_box(None);

        ret
    }

    pub fn update_search_box(&mut self, keyword: Option<&str>) {
        if let Some(keyword) = keyword {
            self.filtered.change_keyword(keyword);
        }

        self.index_map = self
            .filtered
            .matched()
            .with_index()
            .flat_map(|(i, f)| (0..f.faces.len()).map(move |x| (i, x)))
            .collect();

        self.name_width_max = self
            .font_face_names()
            .map(|n| n.len())
            .max()
            .unwrap_or_default();

        if self.len() > 0 {
            self.list_state.borrow_mut().select(Some(0));
        } else {
            self.list_state.borrow_mut().select(None);
        }
    }

    fn cache_index(&self) -> Option<(usize, usize)> {
        Some(self.index_map[self.index()?])
    }

    fn cache_key(&self, width: u32, height: u32) -> Option<CacheKey> {
        Some(CacheKey {
            index: self.cache_index()?,
            rt: self.rt,
            width,
            height,
        })
    }

    pub fn len(&self) -> usize {
        self.index_map.len()
    }

    pub fn render(&self) -> Option<Rc<Result<GlyphCache, &'static str>>> {
        let (width, height) = match self.rt {
            RenderType::Mono => self.get_canvas_size_by_pixel(),
            _ => self.get_canvas_size_by_char(),
        };

        let key = self.cache_key(width, height)?;

        let glyph = self
            .cache
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| {
                Rc::new(
                    self.real_render(width, height)
                        .expect("render can't be null because cache key exist"),
                )
            })
            .clone();

        Some(glyph)
    }

    fn rasterize(&self, _width: u32, height: u32) -> Option<Result<Bitmap, &'static str>> {
        let info = self.current_font_face()?;

        let scale = if matches!(self.rt, RenderType::AsciiLevel10 | RenderType::AsciiLevel70) {
            Some(2.0)
        } else {
            None
        };

        let bitmap: Result<Bitmap, &'static str> = loader::database()
            .with_face_data(info.id, |data, index| -> Result<Bitmap, &'static str> {
                let mut r = Rasterizer::new(data, index).map_err(|_| "Can't pare font file")?;
                r.set_pixel_height(height);
                if let Some(scale) = scale {
                    r.set_hscale(scale);
                }
                r.rasterize(info.gid)
                    .ok_or("Can't get target glyph from this font")
            })
            .unwrap_or(Err("Can't read font file"));

        Some(bitmap)
    }

    fn real_render(&self, width: u32, height: u32) -> Option<Result<GlyphCache, &'static str>> {
        let bitmap = self.rasterize(width, height)?;

        let cache = bitmap.map(|bitmap| match self.rt {
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
        });

        Some(cache)
    }

    fn get_font_face(&self, (i, x): (usize, usize)) -> &'a FaceInfo {
        self.filtered.data()[i].faces[x]
    }

    fn current_font_face(&self) -> Option<&'a FaceInfo> {
        Some(self.get_font_face(self.cache_index()?))
    }

    pub fn current_name(&self) -> Option<&str> {
        Some(&self.current_font_face()?.name)
    }

    pub fn name_width_max(&self) -> usize {
        self.name_width_max
    }

    pub fn font_face_names(&self) -> impl Iterator<Item = &str> {
        self.index_map
            .iter()
            .copied()
            .map(|index| self.get_font_face(index).name.as_ref())
    }

    pub fn mut_list_state(&self) -> RefMut<'_, ListState> {
        self.list_state.borrow_mut()
    }

    pub fn index(&self) -> Option<usize> {
        self.list_state.borrow().selected()
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
        let changed = self
            .list_state
            .borrow()
            .selected()
            .map(|index| index.saturating_add(1).min(self.len().saturating_sub(1)));
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
