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

#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions)]

mod args;
mod font;
mod ft;
mod one_char;
mod preview;

use {
    font::{
        matcher::{FontMatcher, FontSet},
        SortedFamilies,
    },
    preview::{browser::ServerBuilder as PreviewServerBuilder, terminal::ui::UI},
    std::{cmp::Reverse, io::Write, iter::FromIterator, net::SocketAddr, process::exit},
};

fn main() {
    let argument = args::get();

    FontSet::init().unwrap_or_else(|_| {
        eprintln!("init FontConfig failed");
        exit(1);
    });

    let font_set = FontSet::fonts_contains(argument.char.0);

    let families = font::SortedFamilies::from(&font_set);

    if families.is_empty() {
        println!("No font support this character.");
        return;
    }

    if argument.tui {
        let mut ft_library = ft::Library::new().unwrap_or_else(|e| {
            eprintln!("init FreeType failed: {}", e);
            exit(2);
        });
        let ui = UI::new(argument.char.0, families, &mut ft_library).unwrap();
        ui.show().unwrap_or_else(|err| {
            eprintln!("{:?}", err);
        });
    } else {
        let server = if argument.preview {
            Some(PreviewServerBuilder::from_iter(families.iter()).build_for(argument.char.0))
        } else {
            None
        };

        println!("Font(s) support the character {}:", argument.char.description());
        show_font_list(families, argument.verbose);

        if let Some(server) = server {
            server.run_until(show_preview_addr_and_wait);
        }
    }

    FontSet::finalize();
}

fn show_preview_addr_and_wait(addr: SocketAddr) {
    println!("{}", "-".repeat(40));
    println!("Please visit http://{}/ in your browser for preview", addr);
    print!("And press Enter after your finish...");
    std::io::stdout().flush().unwrap();

    // Wait until user input any character before stop the server
    let mut line = " ".to_string();
    std::io::stdin().read_line(&mut line).unwrap();
}

fn show_font_list(families: SortedFamilies<'_>, verbose: bool) {
    let max_len =
        if verbose { 0 } else { families.iter().map(|f| f.name_width).max().unwrap_or_default() };

    families.into_iter().for_each(|mut family| {
        if verbose {
            println!("{}", family.name);
            while let Some(Reverse(face)) = family.fonts.pop() {
                println!("    {}", face.fullname);
            }
        } else {
            println!(
                "{:<family_name_length$} with {} style{}",
                family.name,
                family.styles_count(),
                if family.styles_count() > 1 { "s" } else { "" },
                family_name_length = max_len,
            );
        }
    });
}
