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

#![deny(clippy::all)]
#![deny(warnings)]
#![deny(missing_debug_implementations, rust_2018_idioms, unsafe_code)]

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

use family::{group_by_family_sort_by_name, Family};
use preview::{browser::ServerBuilder as PreviewServerBuilder, terminal::ui::UI};

fn main() {
    let argument = args::get();

    let font_set = loader::faces_contains(argument.char.0);

    let families = group_by_family_sort_by_name(&font_set);

    if families.is_empty() {
        println!("No font support this character.");
        return;
    }

    if argument.tui {
        let ui = UI::new(families).unwrap();
        ui.show().unwrap_or_else(|err| {
            eprintln!("{:?}", err);
        });
    } else {
        let builder = if argument.preview {
            Some(PreviewServerBuilder::from_iter(families.iter()))
        } else {
            None
        };

        println!("Font(s) support the character {}:", argument.char.description());
        show_font_list(families, argument.verbose);

        if let Some(builder) = builder {
            builder.build_for(argument.char.0).run_until(show_preview_addr_and_wait);
        }
    }
}

fn show_preview_addr_and_wait(addr: SocketAddr) {
    println!("{}", "-".repeat(40));
    println!("Please visit http://{}/ in your browser for preview", addr);
    print!("And press Enter after your finish...");
    std::io::stdout().flush().unwrap();

    // Wait until user input any character before stop the server
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}

fn show_font_list(families: Vec<Family<'_>>, verbose: u8) {
    let info = families[0].faces[0];

    let max_len = if verbose > 0 {
        0
    } else {
        families.iter().map(|f| f.default_name_width).max().unwrap_or_default()
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

    let bitmap = loader::DATABASE
        .with_face_data(info.id, |data, index| -> Option<rasterizer::Bitmap> {
            let mut face = rasterizer::FontFace::new(data, index).ok()?;
            face.set_cell_pixel(20, 20);
            face.load_glyph(info.gid.0, rasterizer::PixelFormat::Monochrome)
        })
        .unwrap()
        .unwrap();

    println!("bitmap metrics: {:?}", bitmap.get_metrics());
}
