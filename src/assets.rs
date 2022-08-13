use super::*;

mod pixel_texture;

pub use pixel_texture::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub healer: Rc<PixelTexture>,
}

impl Assets {
    pub fn process(&mut self, _geng: &Geng) {}
}
