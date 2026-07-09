// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2025 7sDream <i@7sdre.am> and contributors
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

use skrifa::{outline::DrawError, raw::ReadError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // #[error("Can't load font data")]
    // CantLoadFontData,
    #[error("Font face has no family name")]
    MissingFamilyName,
    #[error("Missing glyph data")]
    MissingGlyphData,
    #[error("Render target glyph failed: {0}")]
    RasterizerFailed(
        #[source]
        #[from]
        DrawError,
    ),
    #[error("Parse font data failed: {0}")]
    RawFontParseFailed(
        #[source]
        #[from]
        ReadError,
    ),
}
