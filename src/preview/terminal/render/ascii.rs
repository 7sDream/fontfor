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

use super::Render;

static LEVEL10RAMP: &str = " .:-=+*#%@";
static LEVEL70RAMP: &str =
    " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Copy, Clone)]
pub enum AsciiRenders {
    Level10,
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
        let multiplier = (level as f64) / (f64::from(u8::MAX) + 1.0);
        Self { ramp, multiplier }
    }
}

impl Render for AsciiRender {
    type Pixel = char;

    fn render_pixel(&self, _up: u8, _left: u8, gray: u8, _right: u8, _down: u8) -> Self::Pixel {
        let index = (f64::from(gray) * self.multiplier).floor() as usize;
        self.ramp[index]
    }
}
