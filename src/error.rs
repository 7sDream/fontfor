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
