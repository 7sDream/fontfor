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

use {super::ObjectSet, once_cell::unsync::Lazy};

pub const FC_FAMILY: &str = "family";
pub const FC_FULLNAME: &str = "fullname";
pub const FC_FAMILY_LANG: &str = "familylang";
pub const FC_FULLNAME_LANG: &str = "fullnamelang";
// TODO: Figure out why we do not need join the rel path with FontConfig's `sysroot`
pub const FC_FILE: &str = "file";
pub const FC_INDEX: &str = "index";

thread_local! {
    pub static THE_OBJECT_SET: Lazy<ObjectSet> = Lazy::new(|| {
        ObjectSet::default()
            .add(FC_FAMILY)
            .add(FC_FULLNAME)
            .add(FC_FAMILY_LANG)
            .add(FC_FULLNAME_LANG)
            .add(FC_FILE)
            .add(FC_INDEX)
    });
}
