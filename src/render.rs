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

        {
            let mouse = self.geng.window().mouse_pos().map(|x| x as f32);
            let mouse = camera.screen_to_world(framebuffer.size().map(|x| x as f32), mouse);
            draw_2d::Text::unit(
                self.geng.default_font().clone(),
                format!("Mouse at ({:.2}, {:.2})", mouse.x, mouse.y),
                Color::WHITE,
            )
            .draw_2d(geng, framebuffer, camera);
        }

        // Draw units
        for unit in &model.units {
            draw_sprite(
                unit.animation_state.get_sprite(),
                unit.position,
                unit.flip_sprite,
                0.0,
                geng,
                framebuffer,
                camera,
            );
            if let Some(render) = &unit.extra_render {
                match render {
                    ExtraUnitRender::Tank {
                        hand_pos,
                        weapon_pos,
                        rotation,
                    } => {
                        draw_sprite(
                            &Sprite {
                                texture: self.assets.mech.tank.hand.texture(),
                                size: vec2(2.0, 2.0 * 9.0 / 62.0),
                            },
                            unit.position + *hand_pos,
                            unit.flip_sprite,
                            rotation.as_f32(),
                            geng,
                            framebuffer,
                            camera,
                        );
                        draw_sprite(
                            &Sprite {
                                texture: self.assets.mech.tank.weapon.texture(),
                                size: vec2(1.0, 1.0 * 14.0 / 31.0),
                            },
                            unit.position + *hand_pos + weapon_pos.rotate(*rotation),
                            unit.flip_sprite,
                            rotation.as_f32() + f32::PI / 2.0,
                            geng,
                            framebuffer,
                            camera,
                        );
                    }
                }
            }
        }

        // Draw projectiles
        for projectile in &model.projectiles {
            draw_2d::Ellipse::circle(projectile.position.map(|x| x.as_f32()), 0.1, Color::RED)
                .draw_2d(geng, framebuffer, camera);
        }
    }
}

fn draw_sprite(
    sprite: &Sprite,
    position: Position,
    flip: bool,
    rotation: f32,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    let mut aabb = AABB::ZERO.extend_symmetric(sprite.size / 2.0);
    if flip {
        aabb = flip_aabb(aabb);
    }
    draw_2d::TexturedQuad::new(aabb, &*sprite.texture.clone()).draw_2d_transformed(
        geng,
        framebuffer,
        camera,
        Mat3::translate(position.map(|x| x.as_f32())) * Mat3::rotate(rotation),
    );
}

fn flip_aabb<T: Num>(aabb: AABB<T>) -> AABB<T> {
    AABB::point(aabb.bottom_right()).extend_positive(vec2(-aabb.size().x, aabb.size().y))
}
