use thiserror::Error;
use ttf_parser::{FaceParsingError, Tag};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Font face has no family name")]
    MissingFamilyName,
    #[error("Font face has no {0} table")]
    MissingRequiredTable(Tag),
    #[error("Parse {0} table of this font failed")]
    ParseTableFailed(Tag),
    #[error("Parse font face failed: {0}")]
    RawFontParseFailed(
        #[source]
        #[from]
        FaceParsingError,
    ),
}

pub const NAME_TAG: Tag = Tag::from_bytes(b"name");
pub const MISSING_NAME_TABLE: Error = Error::MissingRequiredTable(NAME_TAG);
pub const BROKEN_NAME_TABLE: Error = Error::ParseTableFailed(NAME_TAG);

pub const CMAP_TAG: Tag = Tag::from_bytes(b"cmap");
pub const MISSING_CMAP_TABLE: Error = Error::MissingRequiredTable(CMAP_TAG);
pub const BROKEN_CMAP_TABLE: Error = Error::ParseTableFailed(CMAP_TAG);
