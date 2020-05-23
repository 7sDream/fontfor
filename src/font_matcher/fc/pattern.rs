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

use {super::Charset, fontconfig::fontconfig as fc, std::ffi, std::os::raw::c_char};

pub struct Pattern {
    pub(super) ptr: *mut fc::FcPattern,
}

impl Default for Pattern {
    fn default() -> Self {
        unsafe { Self::from_ptr(fc::FcPatternCreate()) }
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe {
            fc::FcPatternDestroy(self.ptr);
        }
    }
}

impl Pattern {
    /// ## Safety
    ///
    /// the ptr must be
    ///
    /// - point to a valid `FcPattern` struct
    /// - create from functions of `fontconfig` lib which do the RC thing correctly
    const unsafe fn from_ptr(ptr: *mut fc::FcPattern) -> Self {
        Self { ptr }
    }

    #[allow(dead_code)]
    pub fn new(s: &str) -> Result<Self, ()> {
        let c_pattern = ffi::CString::new(s).unwrap();
        let pattern = unsafe { fc::FcNameParse(c_pattern.as_ptr() as *const fc::FcChar8) };
        if pattern.is_null() {
            Err(())
        } else {
            Ok(unsafe { Self::from_ptr(pattern) })
        }
    }

    #[allow(unused_mut)] // In deed, we changed the underlying pointer's target struct
    pub fn add_charset(mut self, charset: &Charset) -> Self {
        let name = ffi::CString::new("charset").unwrap();
        unsafe {
            fc::FcPatternAddCharSet(self.ptr, name.as_ptr(), charset.ptr);
        }
        self
    }
}

impl ToString for Pattern {
    fn to_string(&self) -> String {
        let s = unsafe { fc::FcNameUnparse(self.ptr) };
        let s = unsafe { ffi::CString::from_raw(s as *mut c_char) };
        s.to_string_lossy().to_string()
    }
}
