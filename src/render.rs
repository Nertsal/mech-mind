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
        self.draw_world(model, framebuffer);
        self.draw_ui(model, framebuffer);
    }

    fn draw_world(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let geng = &self.geng;
        let camera = &model.camera;

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
                        ..
                    } => {
                        let mut rotation = *rotation;
                        let mut hand = *hand_pos;
                        if unit.flip_sprite {
                            rotation = -rotation;
                            hand.x = -hand.x;
                        }
                        draw_sprite(
                            &Sprite {
                                texture: self.assets.mech.tank.hand.texture(),
                                size: vec2(2.0, 2.0 * 9.0 / 62.0),
                            },
                            unit.position + hand,
                            unit.flip_sprite,
                            rotation.as_f32(),
                            geng,
                            framebuffer,
                            camera,
                        );
                        let mut weapon = *weapon_pos;
                        if unit.flip_sprite {
                            weapon.x = -weapon.x;
                        }
                        draw_sprite(
                            &Sprite {
                                texture: self.assets.mech.tank.weapon.texture(),
                                size: vec2(1.0, 1.0 * 14.0 / 31.0),
                            },
                            unit.position + hand + weapon.rotate(rotation),
                            unit.flip_sprite,
                            rotation.as_f32(),
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
            let rotation = projectile.velocity.arg();
            draw_sprite(
                projectile.animation_state.get_sprite(),
                projectile.position,
                false,
                rotation.as_f32(),
                geng,
                framebuffer,
                camera,
            );
        }

        // Draw particles
        for particle in &model.particles {
            draw_sprite(
                particle.animation_state.get_sprite(),
                particle.position,
                false,
                0.0,
                geng,
                framebuffer,
                camera,
            );
        }

        // Draw coins
        for coin in &model.coins {
            draw_2d::Ellipse::circle(
                coin.position.map(|x| x.as_f32()),
                coin.radius.as_f32(),
                Color::YELLOW,
            )
            .draw_2d(geng, framebuffer, camera);
        }
    }

    fn draw_ui(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let geng = &self.geng;
        let camera = &geng::PixelPerfectCamera;
        let screen = AABB::ZERO.extend_positive(framebuffer.size().map(|x| x as f32));

        // Currency
        const CURRENCY_FONT_SIZE: f32 = 20.0;
        let currency_pos = vec2(screen.center().x, screen.y_max - CURRENCY_FONT_SIZE * 2.0);
        draw_2d::Text::unit(
            self.geng.default_font().clone(),
            &format!("Coins: {}", model.player_coins),
            Color::WHITE,
        )
        .scale_uniform(CURRENCY_FONT_SIZE)
        .translate(currency_pos)
        .draw_2d(geng, framebuffer, camera);
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
