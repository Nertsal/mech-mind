use super::*;
use geng::{Camera2d, Draw2d};

use model::*;

const FOV: f32 = 20.0;

#[allow(dead_code)]
pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    background: Background,
    last_cam_pos: Coord,
}

pub struct Background {
    objects: Vec<Sprite>,
    placed: Vec<(Position, Coord, Sprite)>,
}

impl Background {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            placed: vec![],
            objects: vec![
                Sprite::new(&assets.background.pillar1, 0.03),
                Sprite::new(&assets.background.pillar2, 0.03),
                Sprite::new(&assets.background.tower, 0.03),
                Sprite::new(&assets.background.town, 0.03),
            ],
        }
    }

    pub fn update(&mut self, max: Coord, delta_pos: Coord) {
        // Move objects back
        for (pos, speed, _) in &mut self.placed {
            pos.x -= *speed * delta_pos;
        }

        // Remove objects that are out of view
        self.placed
            .retain(|(pos, _, sprite)| pos.x + Coord::new(sprite.size.x / 2.0) >= Coord::ZERO);

        // Place new objects
        let mut placed_max = self
            .placed
            .iter()
            .map(|(pos, _, sprite)| pos.x + Coord::new(sprite.size.x / 2.0))
            .max()
            .unwrap_or(Coord::ZERO);
        let mut rng = global_rng();
        while placed_max < max {
            match self.objects.choose(&mut rng) {
                None => break,
                Some(object) => {
                    let offset = rng.gen_range(Coord::new(0.5)..=Coord::new(2.0));
                    placed_max += offset + Coord::new(object.size.x);
                    let position = vec2(
                        placed_max - Coord::new(object.size.x / 2.0),
                        Coord::new(object.size.y / 2.0),
                    );
                    let speed = Coord::new(1.0);
                    self.placed.push((position, speed, object.clone()));
                }
            }
        }
    }
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
        for (pos, _, sprite) in &self.background.placed {
            let pos = *pos + vec2(model.left_border, model.ground_level);
            draw_sprite(
                sprite,
                pos,
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
