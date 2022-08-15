use std::collections::VecDeque;

use crate::game::{PlayerEvent, MechType};

use super::*;
use geng::{Camera2d, Draw2d};

use model::*;

mod background;
mod repeating;

use background::*;
use repeating::*;

const FOV: f32 = 20.0;
const NORMAL_COLOR: Rgba<f32> = Rgba {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
const HOVERED_COLOR: Rgba<f32> = Rgba {
    r: 0.7,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};

#[allow(dead_code)]
pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    background: Background,
    last_cam_pos: Coord,
    pub visualize_hitboxes: bool,
    artillery_slot: AABB<f32>,
    tank_slot: AABB<f32>,
    healer_slot: AABB<f32>,
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
            artillery_slot: AABB::ZERO,
            tank_slot: AABB::ZERO,
            healer_slot: AABB::ZERO,
        }
    }

    pub fn handle_event(&mut self, event: geng::Event) -> Vec<PlayerEvent> {
        let mut events = Vec::new();
        match event {
            geng::Event::MouseDown { position, button: geng::MouseButton::Left } => {
                let pos = position.map(|x| x as f32);
                if self.artillery_slot.contains(pos) {
                    events.push(PlayerEvent::SpawnMech(MechType::Artillery));
                }
                if self.tank_slot.contains(pos) {
                    events.push(PlayerEvent::SpawnMech(MechType::Tank));
                }
                if self.healer_slot.contains(pos) {
                    events.push(PlayerEvent::SpawnMech(MechType::Healer));
                }
            }
            _ => {}
        }
        events
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
                            Rgba::GREEN,
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
            match unit.faction {
                Faction::Mech => {
                    let sprite = Sprite::new(&self.assets.ui.mech_bar, 0.03);
                    let position = unit.position
                        + vec2(
                            0.0,
                            (unit.animation_state.get_sprite().size.y + sprite.size.y) / 2.0,
                        )
                        .map(Coord::new);
                    // Health bar
                    let bar_aabb = layout_bar(
                        AABB {
                            x_min: 4.0 / 35.0,
                            x_max: 31.0 / 35.0,
                            y_min: 1.0 - 19.0 / 27.0,
                            y_max: 1.0 - 15.0 / 27.0,
                        },
                        sprite.size,
                        position.map(|x| x.as_f32()),
                        unit.health.ratio().as_f32(),
                    );
                    let color = Rgba::try_from("#28f000").unwrap();
                    draw_2d::Quad::new(bar_aabb, color).draw_2d(geng, framebuffer, camera);
                    // Sanity bar
                    let bar_aabb = layout_bar(
                        AABB {
                            x_min: 7.0 / 35.0,
                            x_max: 28.0 / 35.0,
                            y_min: 1.0 - 26.0 / 27.0,
                            y_max: 1.0 - 23.0 / 27.0,
                        },
                        sprite.size,
                        position.map(|x| x.as_f32()),
                        unit.sanity
                            .as_ref()
                            .map(|sanity| sanity.ratio().as_f32())
                            .unwrap_or(1.0),
                    );
                    let color = Rgba::try_from("#d77bba").unwrap();
                    draw_2d::Quad::new(bar_aabb, color).draw_2d(geng, framebuffer, camera);
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
                    let bar_aabb = layout_bar(
                        AABB {
                            x_min: 6.0 / 34.0,
                            x_max: 33.0 / 34.0,
                            y_min: 1.0 - 13.0 / 18.0,
                            y_max: 1.0 - 9.0 / 18.0,
                        },
                        sprite.size,
                        position.map(|x| x.as_f32()),
                        unit.health.ratio().as_f32(),
                    );
                    let color = Rgba::try_from("#ac3232").unwrap();
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

        // Energy
        let energy_sprite = Sprite::new(&self.assets.ui.energy_bar, 5.0);
        let position = vec2(0.5 - 100.0 / 258.0, 110.0 / 158.0 - 0.5) * energy_sprite.size
            + vec2(screen.center().x, screen.y_max);
        let bar_aabb = layout_bar(
            AABB {
                x_min: 84.0 / 258.0,
                x_max: 115.0 / 258.0,
                y_min: 1.0 - 132.0 / 158.0,
                y_max: 1.0 - 122.0 / 158.0,
            },
            energy_sprite.size,
            position,
            model.player_energy.ratio().as_f32(),
        );
        let color = Rgba::try_from("#2BD9FE").unwrap();
        draw_2d::Quad::new(bar_aabb, color).draw_2d(geng, framebuffer, camera);
        draw_sprite(
            &energy_sprite,
            position.map(Coord::new),
            false,
            0.0,
            geng,
            framebuffer,
            camera,
        );

        // Slots
        let energy_size = energy_sprite.size / vec2(258.0, -158.0);
        let energy_pos = position + vec2(-energy_sprite.size.x, energy_sprite.size.y) / 2.0;
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);

        // Artillery
        let sprite = Sprite::new(&self.assets.ui.artillery_slot, 4.0);
        let back = self.assets.ui.artillery_slot_bg.texture();
        let position = vec2(150.0, 130.0) * energy_size + energy_pos;
        let aabb = AABB::point(position).extend_symmetric(sprite.size / 2.0);
        let color = if aabb.contains(mouse_pos) {
            HOVERED_COLOR
        } else {
            NORMAL_COLOR
        };
        draw_2d::TexturedQuad::colored(aabb, back, color).draw_2d(geng, framebuffer, camera);
        draw_2d::TexturedQuad::new(aabb, sprite.texture).draw_2d(geng, framebuffer, camera);
        self.artillery_slot = aabb;

        // Tank
        let sprite = Sprite::new(&self.assets.ui.tank_slot, 4.0);
        let back = self.assets.ui.tank_slot_bg.texture();
        let position = vec2(175.0, 130.0) * energy_size + energy_pos;
        let aabb = AABB::point(position).extend_symmetric(sprite.size / 2.0);
        let color = if aabb.contains(mouse_pos) {
            HOVERED_COLOR
        } else {
            NORMAL_COLOR
        };
        draw_2d::TexturedQuad::colored(aabb, back, color).draw_2d(geng, framebuffer, camera);
        draw_2d::TexturedQuad::new(aabb, sprite.texture).draw_2d(geng, framebuffer, camera);
        self.tank_slot = aabb;

        // Healer
        let sprite = Sprite::new(&self.assets.ui.healer_slot, 4.0);
        let back = self.assets.ui.healer_slot_bg.texture();
        let position = vec2(200.0, 130.0) * energy_size + energy_pos;
        let aabb = AABB::point(position).extend_symmetric(sprite.size / 2.0);
        let color = if aabb.contains(mouse_pos) {
            HOVERED_COLOR
        } else {
            NORMAL_COLOR
        };
        draw_2d::TexturedQuad::colored(aabb, back, color).draw_2d(geng, framebuffer, camera);
        draw_2d::TexturedQuad::new(aabb, sprite.texture).draw_2d(geng, framebuffer, camera);
        self.healer_slot = aabb;
    }
}

fn layout_bar(aabb: AABB<f32>, sprite: Vec2<f32>, pos: Vec2<f32>, ratio: f32) -> AABB<f32> {
    let pos = pos - sprite / 2.0;
    let mut aabb = AABB {
        x_min: aabb.x_min * sprite.x + pos.x,
        x_max: aabb.x_max * sprite.x + pos.x,
        y_min: aabb.y_min * sprite.y + pos.y,
        y_max: aabb.y_max * sprite.y + pos.y,
    };
    aabb.x_max = aabb.x_min + aabb.width() * ratio;
    aabb
}

fn draw_aabb_frame(
    aabb: AABB<Coord>,
    width: Coord,
    color: Rgba<f32>,
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
