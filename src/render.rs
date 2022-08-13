use super::*;

use geng::Draw2d;
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

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let geng = &self.geng;
        let assets = &self.assets;
        let camera = &model.camera;

        // Draw mechs
        for mech in &model.mechs {
            draw_2d::TexturedQuad::new(
                AABB::point(mech.position)
                    .extend_uniform(mech.size)
                    .map(|x| x.as_f32()),
                &*assets.healer_mech.idle.clone(),
            )
            .draw_2d(geng, framebuffer, camera);
        }

        // Draw enemies
        for enemy in &model.enemies {
            draw_2d::Quad::new(
                AABB::point(enemy.position)
                    .extend_uniform(enemy.size)
                    .map(|x| x.as_f32()),
                Color::RED,
            )
            .draw_2d(geng, framebuffer, camera);
        }
    }
}
