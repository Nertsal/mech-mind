use super::*;

use model::*;

#[allow(dead_code)]
pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(&mut self, _model: &Model, _framebuffer: &mut ugli::Framebuffer) {}
}
