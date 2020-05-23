use super::{matcher::ct::FontSet, SortedFamilies};

impl<'fs> From<&'fs FontSet> for SortedFamilies<'fs> {
    fn from(_: &'fs FontSet) -> Self {
        SortedFamilies(Vec::new())
    }
}
