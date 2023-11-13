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

use clap::Parser;

use super::one_char::OneChar;

#[derive(clap::Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Args {
    /// Verbose mode, -v show all font styles, -vv adds file and font face index
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Preview character use supported fonts in browser
    #[arg(short, long)]
    pub preview: bool,

    /// Enable Terminal UI mode.
    /// enable this mode will disable the --preview/-p and ignore --verbose/-v option
    #[arg(short, long)]
    pub tui: bool,

    /// The character
    #[arg(name = "CHAR")]
    pub char: OneChar,
}

pub fn get() -> Args {
    Args::parse()
}
