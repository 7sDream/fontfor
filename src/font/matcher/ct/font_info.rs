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

use {
    crate::font::Font, core_foundation::base::ItemRef,
    core_text::font_descriptor::CTFontDescriptor, std::borrow::Cow, std::convert::TryFrom,
};

pub struct FontInfo<'fs> {
    pub(super) desc: ItemRef<'fs, CTFontDescriptor>,
}

impl<'fs> FontInfo<'fs> {
    // TODO: Figure out how to get font face index in file from FontDescriptor
    #[allow(clippy::unused_self)]
    const fn font_face_index(&self) -> Option<usize> {
        Some(0)
    }
}

impl<'fi> TryFrom<FontInfo<'fi>> for Font<'fi> {
    type Error = ();

    fn try_from(font_info: FontInfo<'fi>) -> Result<Self, Self::Error> {
        let family_name = Cow::from(font_info.desc.family_name());
        let fullname = Cow::from(font_info.desc.display_name());
        let path = Cow::from(
            font_info.desc.font_path().ok_or(())?.into_os_string().into_string().map_err(|_| ())?,
        );
        let index = font_info.font_face_index().ok_or(())?;
        Ok(Font { family_name, fullname, path, index })
    }
}
