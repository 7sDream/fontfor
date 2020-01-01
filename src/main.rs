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
mod one_char;

use {std::collections::HashMap, unicode_width::UnicodeWidthStr};

fn main() {
    let argument = args::get();

    fc::init().expect("init fontconfig failed");

    let charset = fc::Charset::default().add_char(argument.char.0);
    let pattern = fc::Pattern::default().add_charset(charset);

    println!("Fonts support the character {}: ", argument.char.description());

    let mut families: HashMap<(String, usize), u32> = HashMap::new();

    fc::FontSet::match_pattern(&pattern)
        .fonts()
        .map(|font| font.family())
        .filter_map(|mut family| family.pop())
        // TODO: figure out the meaning of prefix dot
        .filter(|family| !family.starts_with('.'))
        .for_each(|family| {
            let len = UnicodeWidthStr::width(family.as_str());
            *families.entry((family, len)).or_insert(0) += 1;
        });

    fc::finalize();

    let mut families: Vec<_> = families.into_iter().collect();

    families.sort();

    let max_len = families.iter().map(|((_, len), _)| *len).max().unwrap_or(0);

    for ((family, len), count) in families {
        println!(
            "{}{} with {} style{}",
            family,
            " ".repeat(max_len - len),
            count,
            if count == 1 { "" } else { "s" }
        );
    }
}
