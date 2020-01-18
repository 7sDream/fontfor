use {
    super::SingleThreadServer,
    crate::font::{Family, GetValueByLang},
    std::iter::FromIterator,
};

pub struct Builder<'a> {
    families: Vec<&'a str>,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self { families: vec![] }
    }
}

impl<'iter, 'a: 'iter> FromIterator<&'iter Family<'a>> for Builder<'a> {
    fn from_iter<T: IntoIterator<Item = &'iter Family<'a>>>(iter: T) -> Self {
        let mut builder = Self::default();
        iter.into_iter().for_each(|f| {
            builder.add_family(f);
        });
        builder
    }
}

impl<'a> Builder<'a> {
    #[allow(dead_code)]
    pub fn add_family(&mut self, family: &Family<'a>) -> &mut Self {
        self.families.push(family.name.get_default());
        self
    }

    #[allow(clippy::unused_self)]
    fn build_html(self, c: char) -> String {
        format!(
            include_str!("statics/template.html"),
            style = include_str!("statics/style.css"),
            font_previews = self
                .families
                .into_iter()
                .map(|family| {
                    format!(
                        include_str!("statics/preview_block_template.html"),
                        char = c,
                        family = family
                    )
                })
                .collect::<String>()
        )
    }

    pub fn build_for(self, c: char) -> SingleThreadServer {
        SingleThreadServer::new(self.build_html(c))
    }
}
