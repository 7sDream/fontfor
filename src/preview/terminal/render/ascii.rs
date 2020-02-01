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

use super::Render;

static LEVEL10RAMP: &str = " .:-=+*#%@";
static LEVEL70RAMP: &str =
    " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AsciiRenders {
    #[allow(dead_code)] // TODO: delete this
    Level10,
    #[allow(dead_code)] // TODO: delete this
    Level70,
}

pub struct AsciiRender {
    ramp: Vec<char>,
    multiplier: f64,
}

impl AsciiRender {
    pub fn new(render_type: AsciiRenders) -> Self {
        let s = match render_type {
            AsciiRenders::Level10 => LEVEL10RAMP,
            AsciiRenders::Level70 => LEVEL70RAMP,
        };
        let ramp: Vec<_> = s.chars().collect();
        let level = ramp.len();
        #[allow(clippy::cast_precision_loss)]
        let multiplier = (level as f64) / (f64::from(u8::max_value()) + 1.0);
        Self { ramp, multiplier }
    }
}

impl Render for AsciiRender {
    fn gray_to_char(&self, _up: u8, _left: u8, gray: u8, _right: u8, _down: u8) -> char {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let index = (f64::from(gray) * self.multiplier).floor() as usize;
        self.ramp[index]
    }
}
