use std::borrow::Cow;

pub trait CharRenderResult: Sized {
    type Render: CharRenderer<Result = Self>;

    fn return_render(self) -> Self::Render;
    fn get_height(&self) -> usize;
    fn get_width(&self) -> usize;
    fn get_buffer(&self) -> &[Cow<'_, [u8]>];
}

pub trait CharRenderer: Sized {
    type Result: CharRenderResult<Render = Self>;
    type Error;

    fn set_cell_pixel(&mut self, height: usize, width: usize) -> Result<(), Self::Error>;
    fn render_char(self, c: char, mono: bool) -> Result<Self::Result, (Self, Self::Error)>;
}

pub enum LoaderInput<'a> {
    FreeType(&'a str, usize),
    #[allow(dead_code)]
    CoreText(&'a str),
}

pub trait CharRendererLoader<'i> {
    type Render: CharRenderer;
    type Error;

    fn load_render(&'i self, input: &LoaderInput<'_>) -> Result<Self::Render, Self::Error>;
}
