use {super::SortedFamilies, crate::font_matcher::ct::FontSet};

impl<'fs> From<&'fs FontSet> for SortedFamilies<'fs> {
    fn from(_: &'fs FontSet) -> Self {
        SortedFamilies(Vec::new())
    }
}
