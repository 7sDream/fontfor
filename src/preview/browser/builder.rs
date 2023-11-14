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

use std::{fmt::Write, iter::FromIterator};

use super::SingleThreadServer;
use crate::family::Family;

#[derive(Default)]
pub struct Builder<'a> {
    families: Vec<&'a str>,
}

impl<'a, 'iter> FromIterator<&'iter Family<'a>> for Builder<'a> {
    fn from_iter<T: IntoIterator<Item = &'iter Family<'a>>>(iter: T) -> Self {
        let mut builder = Self::default();
        iter.into_iter().for_each(|f| {
            builder.add_family(f);
        });
        builder
    }
}

impl<'a> Builder<'a> {
    pub fn add_family(&mut self, family: &Family<'a>) -> &mut Self {
        self.families.push(family.name);
        self
    }

    fn build_html(self, c: char) -> String {
        let font_previews =
            self.families.into_iter().fold(String::new(), |mut acc: String, family| {
                write!(
                    &mut acc,
                    include_str!("statics/preview_block_template.html"),
                    char = c,
                    family = family,
                )
                .expect("write to string should always success");
                acc
            });

        format!(
            include_str!("statics/template.html"),
            style = include_str!("statics/style.css"),
            script = include_str!("statics/script.js"),
            font_previews = font_previews,
        )
    }

    pub fn build_for(self, c: char) -> SingleThreadServer {
        SingleThreadServer::new(self.build_html(c))
    }
}
