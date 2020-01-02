use {
    super::fc::{Font, StrByLang},
    std::convert::TryFrom,
    unicode_width::UnicodeWidthStr,
};

const DEFAULT_LANG: &str = "en";

trait GetValueByLang {
    type Item;

    fn get_by_lang(&self, lang: &str) -> Option<&Self::Item>;

    fn when_missing(&self) -> &Self::Item;

    fn get_default(&self) -> &Self::Item {
        if let Some(value) = self.get_by_lang(DEFAULT_LANG) {
            value
        } else {
            self.when_missing()
        }
    }
}

impl<'a> GetValueByLang for StrByLang<'a> {
    type Item = &'a str;

    fn get_by_lang(&self, lang: &str) -> Option<&Self::Item> {
        self.get(lang).and_then(|values| values.iter().next())
    }

    fn when_missing(&self) -> &Self::Item {
        self.values().next().unwrap().iter().next().unwrap()
    }
}

pub struct Family<'a> {
    pub name: StrByLang<'a>,
    pub fonts: Vec<FontInfo<'a>>,
    pub default_name: &'a str,
    pub default_name_width: usize,
}

impl<'a> Family<'a> {
    pub fn new(name: StrByLang<'a>) -> Self {
        let default_name = *name.get_default();
        let default_name_width = UnicodeWidthStr::width(default_name);
        Self { name, fonts: vec![], default_name, default_name_width }
    }

    pub fn styles_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn add_font(&mut self, font: FontInfo<'a>) -> &mut Self {
        self.fonts.push(font);
        self
    }
}

pub struct FontInfo<'a> {
    pub family_names: StrByLang<'a>,
    pub fullnames: StrByLang<'a>,
}

impl<'a> TryFrom<Font<'a>> for FontInfo<'a> {
    type Error = ();

    fn try_from(font: Font<'a>) -> Result<Self, Self::Error> {
        let f = Self { family_names: font.family_names()?, fullnames: font.fullnames()? };
        if f.family_names.is_empty() || f.fullnames.is_empty() {
            Err(())
        } else {
            Ok(f)
        }
    }
}

impl<'a> FontInfo<'a> {
    pub fn default_family_name(&self) -> &&'a str {
        self.family_names.get_default()
    }

    #[allow(dead_code)]
    pub fn default_fullname(&self) -> &&'a str {
        self.fullnames.get_default()
    }
}
