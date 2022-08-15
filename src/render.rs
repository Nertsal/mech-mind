use std::collections::VecDeque;

use super::*;
use geng::{Camera2d, Draw2d};

use model::*;

mod background;
mod repeating;

use background::*;
use repeating::*;

const FOV: f32 = 20.0;

#[allow(dead_code)]
pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    background: Background,
    last_cam_pos: Coord,
    pub visualize_hitboxes: bool,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: FOV,
            },
            background: Background::new(assets),
            last_cam_pos: Coord::ZERO,
            visualize_hitboxes: false,
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let camera_width = self.camera.fov * framebuffer_size.x / framebuffer_size.y;
        self.camera.center.x = model.left_border.as_f32() + camera_width / 2.0;

        self.background.update(
            Coord::new(camera_width),
            Coord::new(self.camera.center.x) - self.last_cam_pos,
        );
        for (pos, sprite) in self
            .background
            .background
            .placed
            .iter()
            .map(|x| {
                (
                    vec2(
                        *x,
                        Coord::new(self.background.background.sprite.size.y / 2.0),
                    ),
                    &self.background.background.sprite,
                )
            })
            .chain(self.background.floor.placed.iter().map(|x| {
                (
                    vec2(*x, -Coord::new(self.background.floor.sprite.size.y / 2.0)),
                    &self.background.floor.sprite,
                )
            }))
            .chain(
                self.background
                    .placed
                    .iter()
                    .map(|(pos, _, sprite)| (*pos, sprite)),
            )
        {
            draw_sprite(
                sprite,
                pos + vec2(model.left_border, model.ground_level),
                false,
                0.0,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }

        self.draw_world(model, framebuffer);
        self.draw_ui(model, framebuffer);

        self.last_cam_pos = Coord::new(self.camera.center.x);
    }

    fn draw_world(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let geng = &self.geng;
        let camera = &self.camera;

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

        // Hitboxes
        if self.visualize_hitboxes {
            for (pos, collider) in model
                .units
                .iter()
                .map(|unit| (unit.position, &unit.collider))
                .chain(
                    model
                        .projectiles
                        .iter()
                        .map(|proj| (proj.position, &proj.collider)),
                )
            {
                match collider {
                    Collider::Aabb { size } => {
                        draw_aabb_frame(
                            AABB::point(pos).extend_symmetric(*size / Coord::new(2.0)),
                            Coord::new(0.1),
                            Color::GREEN,
                            geng,
                            framebuffer,
                            camera,
                        );
                    }
                }
            }
        }

        // Health
        for unit in &model.units {
            fn layout(aabb: AABB<f32>, sprite: Vec2<f32>, pos: Position, ratio: f32) -> AABB<f32> {
                let pos = pos.map(|x| x.as_f32()) - sprite / 2.0;
                let mut aabb = AABB {
                    x_min: aabb.x_min * sprite.x + pos.x,
                    x_max: aabb.x_max * sprite.x + pos.x,
                    y_min: aabb.y_min * sprite.y + pos.y,
                    y_max: aabb.y_max * sprite.y + pos.y,
                };
                aabb.x_max = aabb.x_min + aabb.width() * ratio;
                aabb
            }

            match unit.faction {
                Faction::Mech => {
                    let sprite = Sprite::new(&self.assets.ui.mech_bar, 0.03);
                    let position = unit.position
                        + vec2(
                            0.0,
                            (unit.animation_state.get_sprite().size.y + sprite.size.y) / 2.0,
                        )
                        .map(Coord::new);
                    draw_sprite(&sprite, position, false, 0.0, geng, framebuffer, camera);
                }
                Faction::Alien => {
                    let sprite = Sprite::new(&self.assets.ui.enemy_health, 0.03);
                    let position = unit.position
                        + vec2(
                            0.0,
                            (unit.animation_state.get_sprite().size.y + sprite.size.y) / 2.0,
                        )
                        .map(Coord::new);
                    let bar_aabb = layout(
                        AABB {
                            x_min: 6.0 / 34.0,
                            x_max: 33.0 / 34.0,
                            y_min: 1.0 - 13.0 / 18.0,
                            y_max: 1.0 - 9.0 / 18.0,
                        },
                        sprite.size,
                        position,
                        unit.health.ratio().as_f32(),
                    );
                    let color = Color::try_from("#ac3232").unwrap();
                    draw_2d::Quad::new(bar_aabb, color).draw_2d(geng, framebuffer, camera);
                    draw_sprite(&sprite, position, false, 0.0, geng, framebuffer, camera);
                }
            }
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

fn draw_aabb_frame(
    aabb: AABB<Coord>,
    width: Coord,
    color: Color<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    let aabb = aabb.map(|x| x.as_f32());
    let left_mid = vec2(aabb.x_min, aabb.center().y);
    let chain = Chain::new(vec![
        left_mid,
        aabb.top_left(),
        aabb.top_right(),
        aabb.bottom_right(),
        aabb.bottom_left(),
        left_mid,
    ]);
    draw_2d::Chain::new(chain, width.as_f32(), color, 0).draw_2d(geng, framebuffer, camera);
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
