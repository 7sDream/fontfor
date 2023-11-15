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

#![deny(clippy::all)]
#![deny(warnings)]
#![deny(rust_2018_idioms, unsafe_code)]

mod args;
mod loader;
mod family;
mod rasterizer;
mod one_char;
mod preview;

use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use family::Family;
use preview::{browser::ServerBuilder as PreviewServerBuilder, terminal::ui::UI};

fn main() {
    let argument = args::get();

    loader::init(&argument.custom_font_paths);

    let font_set = loader::query(argument.char.0);
    let families = family::group_by_family_sort_by_name(&font_set);

    if families.is_empty() {
        eprintln!(
            "No font support this character {}.",
            argument.char.description()
        );
        return;
    }

    if argument.tui {
        let ui = UI::new(families).expect("family length checked before, must not empty");
        if let Err(err) = ui.show() {
            eprintln!("{:?}", err);
        };
    } else {
        let builder = if argument.preview {
            Some(PreviewServerBuilder::from_iter(families.iter()))
        } else {
            None
        };

        println!(
            "Font(s) support the character {}:",
            argument.char.description()
        );
        show_font_list(families, argument.verbose);

        if let Some(builder) = builder {
            builder
                .build_for(argument.char.0)
                .run_until(show_preview_addr_and_wait);
        }
    }
}

fn show_preview_addr_and_wait(addr: SocketAddr) {
    println!("{}", "-".repeat(40));
    println!("Please visit http://{}/ in your browser for preview", addr);
    print!("And press Enter after your finish...");
    std::io::stdout()
        .flush()
        .expect("flush stdout should not fail");

    // Wait until user input any character before stop the server
    let _ = std::io::stdin()
        .read(&mut [0u8])
        .expect("read from stdout should not fail");
}

fn show_font_list(families: Vec<Family<'_>>, verbose: u8) {
    let max_len = if verbose > 0 {
        0
    } else {
        families
            .iter()
            .map(|f| f.default_name_width)
            .max()
            .unwrap_or_default()
    };

    families.into_iter().for_each(|family| {
        if verbose > 0 {
            println!("{}", family.name);
            for face in family.faces {
                print!("\t{}", face.name);
                if verbose > 1 {
                    print!("\t{}:{}", face.path.to_string_lossy(), face.index)
                }
                println!()
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
