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
    crate::{
        font::{Font, GetValueByLang, SortedFamilies},
        ft::{FontFace as FtFontFace, Library as FtLibrary},
        preview::terminal::render::{
            AsciiRender, AsciiRenders, MonoRender, MoonRender, Render, RenderResult,
        },
    },
    once_cell::sync::Lazy,
    std::{
        cell::{Cell, RefCell},
        collections::hash_map::HashMap,
        rc::Rc,
    },
    tui::widgets::ListState,
};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RenderType {
    AsciiLevel10,
    AsciiLevel70,
    Moon,
    Mono,
}

static RENDERS: Lazy<HashMap<RenderType, Box<dyn Render + Sync>>> = Lazy::new(|| {
    let mut renders: HashMap<RenderType, Box<dyn Render + Sync>> = HashMap::new();
    renders.insert(RenderType::AsciiLevel10, Box::new(AsciiRender::new(AsciiRenders::Level10)));
    renders.insert(RenderType::AsciiLevel70, Box::new(AsciiRender::new(AsciiRenders::Level70)));
    renders.insert(RenderType::Moon, Box::new(MoonRender::new()));
    renders.insert(RenderType::Mono, Box::new(MonoRender::default()));
    renders
});

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct CacheKey(usize, RenderType, u32, u32);

pub struct State<'fc, 'ft> {
    c: char,
    font_faces_info: Vec<Font<'fc>>,
    font_faces_name: Vec<&'fc str>,
    name_width_max: usize,
    pub list_state: RefCell<ListState>,
    height: Cell<u32>,
    width: Cell<u32>,
    rt: RenderType,
    cache: RefCell<HashMap<CacheKey, Rc<Result<RenderResult, &'static str>>>>,
    font_faces: Vec<Cell<Option<FtFontFace<'ft>>>>,
    ft: &'ft FtLibrary,
}

impl<'fc, 'ft> State<'fc, 'ft> {
    pub fn new(c: char, families: SortedFamilies<'fc>, ft: &'ft FtLibrary) -> Self {
        let font_faces_info: Vec<_> =
            families.into_iter().flat_map(|f| f.fonts.into_iter().map(|r| r.0)).collect();
        let font_faces_name: Vec<_> =
            font_faces_info.iter().map(|f| *f.fullnames.get_default()).collect();
        let name_width_max = font_faces_name.iter().map(|f| f.len()).max().unwrap_or_default();

        let mut font_faces = Vec::new();
        for _ in 0..font_faces_info.len() {
            font_faces.push(Cell::new(None));
        }

        let cache = RefCell::default();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            c,
            font_faces_info,
            font_faces_name,
            name_width_max,
            list_state: RefCell::new(list_state),
            height: Cell::new(0),
            width: Cell::new(0),
            rt: RenderType::Mono,
            cache,
            font_faces,
            ft,
        }
    }

    fn cache_key(&self) -> CacheKey {
        CacheKey(self.index(), self.rt, self.height.get(), self.width.get())
    }

    pub fn render(&self) -> Rc<Result<RenderResult, &'static str>> {
        let key = self.cache_key();
        self.cache.borrow_mut().entry(key).or_insert_with(|| Rc::new(self.real_render())).clone()
    }

    fn get_font_face(&self) -> Result<FtFontFace<'ft>, &'static str> {
        let font_face_slot = self.font_faces.get(self.index()).unwrap();

        font_face_slot
            .take()
            .ok_or(())
            .or_else(|_| {
                let font_info = &self.font_faces_info[self.index()];
                self.ft
                    .load_font(font_info.path, font_info.index.into())
                    .map_err(|_| "Can't load current font")
            })
            .and_then(|font_face| self.set_font_face_size(font_face))
    }

    fn return_font_face(&self, font: FtFontFace<'ft>) {
        let font_face_slot = self.font_faces.get(self.index()).unwrap();
        font_face_slot.set(Some(font));
    }

    fn set_font_face_size(
        &self, mut font_face: FtFontFace<'ft>,
    ) -> Result<FtFontFace<'ft>, &'static str> {
        let height = self.height.get();
        let width = self.width.get();

        font_face
            .set_cell_pixel(height.into(), width.into())
            .map(|_| font_face)
            .map_err(|_| "Current font don't support this size")
    }

    fn real_render(&self) -> Result<RenderResult, &'static str> {
        let font_face = self.get_font_face()?;

        match font_face.load_char(self.c, self.rt == RenderType::Mono) {
            Ok(bitmap) => {
                let render = RENDERS.get(&self.rt).unwrap();
                let result = render.render(&bitmap);
                self.return_font_face(bitmap.return_font_face());
                Ok(result)
            }
            Err((font, _)) => {
                self.return_font_face(font);
                Err("Can't get glyph info from current font")
            }
        }
    }

    pub fn current_name(&self) -> &'fc str {
        self.font_faces_name[self.index()]
    }

    pub const fn name_width_max(&self) -> usize {
        self.name_width_max
    }

    pub const fn family_names(&self) -> &Vec<&'fc str> {
        &self.font_faces_name
    }

    pub fn index(&self) -> usize {
        self.list_state.borrow().selected().unwrap()
    }

    pub fn move_up(&mut self) {
        let changed = self.list_state.borrow().selected().map(|index| index.saturating_sub(1));
        self.list_state.borrow_mut().select(changed);
    }

    pub fn move_down(&mut self) {
        let changed =
            self.list_state.borrow().selected().map(|index| {
                index.saturating_add(1).min(self.font_faces_name.len().saturating_sub(1))
            });
        self.list_state.borrow_mut().select(changed);
    }

    pub const fn get_render_type(&self) -> &RenderType {
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

    pub fn update_char_pixel_cell(&self, width: u32, height: u32) {
        self.width.replace(width);
        self.height.replace(height);
    }

    pub fn get_char_pixel_cell(&self) -> (u32, u32) {
        (self.width.get(), self.height.get())
    }
}
