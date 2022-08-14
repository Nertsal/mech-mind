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
        let camera = &model.camera;

        // Draw units
        for unit in &model.units {
            unit.animation_state
                .get_sprite()
                .draw(unit.position, geng, framebuffer, camera);
        }

        // Draw projectiles
        for projectile in &model.projectiles {
            draw_2d::Ellipse::circle(projectile.position.map(|x| x.as_f32()), 0.1, Color::RED)
                .draw_2d(geng, framebuffer, camera);
        }
    }
}
