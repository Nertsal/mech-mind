use geng::Draw2d;

use super::*;

pub struct Sprite {
    pub texture: Rc<ugli::Texture>,
    pub size: Vec2<f32>,
}

impl Sprite {
    pub fn draw(
        &self,
        position: Position,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl geng::AbstractCamera2d,
    ) {
        draw_2d::TexturedQuad::new(
            AABB::point(position.map(|x| x.as_f32())).extend_symmetric(self.size / 2.0),
            &*self.texture.clone(),
        )
        .draw_2d(geng, framebuffer, camera);
    }
}
