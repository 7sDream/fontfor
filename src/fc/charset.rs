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

use fontconfig::fontconfig as fc;

pub struct Charset {
    pub(super) ptr: *mut fc::FcCharSet,
}

impl Default for Charset {
    fn default() -> Self {
        unsafe { Self::from_ptr(fc::FcCharSetCreate()) }
    }
}

impl Drop for Charset {
    fn drop(&mut self) {
        unsafe {
            fc::FcCharSetDestroy(self.ptr);
        }
    }
}

impl Charset {
    /// ## Safety
    ///
    /// the ptr must be
    ///
    /// - point to a valid `FcCharSet` struct
    /// - create from functions of `fontconfig` lib which do the RC thing correctly
    const unsafe fn from_ptr(ptr: *mut fc::FcCharSet) -> Self {
        Self { ptr }
    }

    #[allow(unused_mut)] // In deed, we changed the underlying pointer's target struct
    pub fn add_char(mut self, c: char) -> Self {
        unsafe {
            fc::FcCharSetAddChar(self.ptr, u32::from(c) as fc::FcChar32);
        }
        self
    }
}
