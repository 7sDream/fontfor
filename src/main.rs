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

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(warnings)]

mod args;
mod fc;
mod font;
mod one_char;
mod preview;

use {
    font::GetValueByLang,
    preview::browser::ServerBuilder as PreviewServerBuilder,
    std::{io::Write, iter::FromIterator},
};

fn main() {
    let argument = args::get();

    fc::init().expect("init fontconfig failed");

    let charset = fc::Charset::default().add_char(argument.char.0);
    let pattern = fc::Pattern::default().add_charset(charset);
    let font_set = fc::FontSet::match_pattern(&pattern);

    let families = font::SortedFamilies::from(&font_set);

    let server = if argument.preview {
        Some(PreviewServerBuilder::from_iter(families.iter()))
    } else {
        None
    };

    let max_len = if argument.verbose {
        0
    } else {
        families.iter().map(|f| f.default_name_width).max().unwrap_or_default()
    };

    println!("Font(s) support the character {}:", argument.char.description());
    families.into_iter().for_each(|family| {
        if argument.verbose {
            println!("{}", family.name.get_default());
            for font in family.fonts.into_sorted_vec() {
                println!("    {}", font.fullnames.get_default());
            }
        } else {
            println!(
                "{:<family_name_length$} with {} style{}",
                family.name.get_default(),
                family.styles_count(),
                if family.styles_count() > 1 { "s" } else { "" },
                family_name_length = max_len,
            );
        }
    });

    if argument.preview {
        let server = server.unwrap().build_for(argument.char.0);
        server.run_until(move |addr| {
            println!("{}", "-".repeat(40));
            println!("Please visit http://{}/ in your browser for preview", addr);
            print!("And press Enter after your finish...");
            std::io::stdout().flush().unwrap();

            // Wait until user input any character before stop the server
            let mut line = " ".to_string();
            std::io::stdin().read_line(&mut line).unwrap();
        });
    }

    fc::finalize();
}
