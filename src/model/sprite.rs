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
        flip: bool,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl geng::AbstractCamera2d,
    ) {
        let mut aabb = AABB::point(position.map(|x| x.as_f32())).extend_symmetric(self.size / 2.0);
        if flip {
            aabb = AABB::point(aabb.bottom_right())
                .extend_positive(vec2(-aabb.size().x, aabb.size().y));
        }
        draw_2d::TexturedQuad::new(aabb, &*self.texture.clone()).draw_2d(geng, framebuffer, camera);
    }
}
