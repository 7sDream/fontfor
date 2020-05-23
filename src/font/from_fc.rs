use {
    super::{
        matcher::fc::{FontInfo, FontSet},
        Family, Font, GetValueByLang, SortedFamilies,
    },
    std::{collections::HashMap, convert::TryFrom},
};

impl<'fi> TryFrom<FontInfo<'fi>> for Font<'fi> {
    type Error = ();

    fn try_from(font_info: FontInfo<'fi>) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_sign_loss)] // Because it is index
        let f = Self {
            family_names: font_info.family_names()?,
            fullnames: font_info.fullnames()?,
            path: font_info.path()?,
            index: font_info.index()? as usize,
        };
        if f.family_names.is_empty() || f.fullnames.is_empty() {
            Err(())
        } else {
            Ok(f)
        }
    }
}

impl<'fs> From<&'fs FontSet> for SortedFamilies<'fs> {
    fn from(font_set: &'fs FontSet) -> Self {
        let mut families = HashMap::new();

        font_set.fonts().for_each(|fc_font| {
            if let Ok(font) = Font::try_from(fc_font) {
                let family = font.family_names.get_default();
                families
                    .entry(*family)
                    .or_insert_with(|| Family::new(font.family_names.clone()))
                    .add_font(font);
            }
        });

        let mut families: Vec<Family<'fs>> =
            families.into_iter().map(|(_, family)| family).collect();

        families.sort_by_key(|f| -> &'fs str { f.name.get_default() });

        Self(families)
    }
}
