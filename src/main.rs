// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 7sDream <i@7sdre.am> and contributors
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

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(warnings)]

mod args;
mod fc;
mod font_info;
mod one_char;

use {
    font_info::{Family, FontInfo},
    std::{collections::HashMap, convert::TryFrom},
};

fn main() {
    let argument = args::get();

    fc::init().expect("init fontconfig failed");

    let charset = fc::Charset::default().add_char(argument.char.0);
    let pattern = fc::Pattern::default().add_charset(charset);

    println!("Fonts support the character {}: ", argument.char.description());

    let mut families: HashMap<&str, Family> = HashMap::new();

    let matches = fc::FontSet::match_pattern(&pattern);

    matches.fonts().for_each(|fc_font| {
        if let Ok(font) = FontInfo::try_from(fc_font) {
            let family = font.default_family_name();
            families
                .entry(*family)
                .or_insert_with(|| Family::new(font.family_names.clone()))
                .add_font(font);
        }
    });

    let max_len =
        families.values().map(|family| family.default_name_width).max().unwrap_or_default();

    let mut families: Vec<_> = families.into_iter().collect();

    families.sort_by_key(|v| v.0); // Sort by name

    for (name, family) in families {
        println!(
            "{}{} with {} style{}",
            name,
            " ".repeat(max_len - family.default_name_width),
            family.styles_count(),
            if family.styles_count() > 1 { "s" } else { "" }
        );
    }

    fc::finalize();
}
