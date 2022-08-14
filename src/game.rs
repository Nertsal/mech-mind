use super::*;

use model::*;
use render::Render;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: Render,
    model: Model,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: Render::new(geng, assets),
            model: Model::new(),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::G => {
                    let animation = to_animation(
                        &self.assets.mech.tank.attack,
                        vec2(2.0, 2.0),
                        Time::ONE,
                        Some((
                            2,
                            Effect::Projectile(Box::new(ProjectileEffect {
                                offset: Position::ZERO,
                                speed: Coord::new(10.0),
                                on_hit: Effect::Damage(Box::new(DamageEffect {
                                    damage_type: DamageType::Physical,
                                    value: Hp::new(1.0),
                                })),
                            })),
                        )),
                    );
                    self.model.units.insert(Unit {
                        id: self.model.id_gen.gen(),
                        faction: Faction::Mech,
                        ai: UnitAI::Engage(TargetAI::Closest),
                        health: Health::new(Hp::new(10.0)),
                        sanity: Some(Health::new(Hp::new(100.0))),
                        collider: Collider::Aabb {
                            size: vec2(1.0, 2.0).map(Coord::new),
                        },
                        position: vec2(0.0, 5.0).map(Coord::new),
                        velocity: Velocity::ZERO,
                        speed: Coord::new(2.0),
                        acceleration: Coord::new(10.0),
                        target_velocity: Velocity::ZERO,
                        animation_state: AnimationState::new(&animation).0,
                        action: Action {
                            cooldown: Time::new(1.0),
                            engage_radius: Coord::new(10.0),
                            animation,
                        },
                        action_state: ActionState::Ready,
                    });
                }
                geng::Key::H => {
                    let animation = to_animation(
                        &self.assets.mech.artillery.attack,
                        vec2(2.0, 2.0),
                        Time::ONE,
                        None,
                    );
                    self.model.units.insert(Unit {
                        id: self.model.id_gen.gen(),
                        faction: Faction::Alien,
                        ai: UnitAI::Idle,
                        health: Health::new(Hp::new(10.0)),
                        sanity: None,
                        collider: Collider::Aabb {
                            size: vec2(1.0, 2.0).map(Coord::new),
                        },
                        position: vec2(5.0, 5.0).map(Coord::new),
                        velocity: Velocity::ZERO,
                        speed: Coord::new(2.0),
                        acceleration: Coord::new(10.0),
                        target_velocity: Velocity::ZERO,
                        animation_state: AnimationState::new(&animation).0,
                        action: Action {
                            cooldown: Time::new(1.0),
                            engage_radius: Coord::new(2.0),
                            animation,
                        },
                        action_state: ActionState::Ready,
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);
        self.model.update(delta_time);
    }
}

fn to_animation(
    textures: &[assets::PixelTexture],
    sprite_size: Vec2<f32>,
    cycle_time: Time,
    action: Option<(usize, Effect)>,
) -> Rc<Animation> {
    Rc::new(Animation {
        keyframes: {
            let time = cycle_time / Time::new(textures.len() as f32);
            let attack_frame = 2;
            textures
                .iter()
                .enumerate()
                .map(|(frame, texture)| AnimationFrame {
                    sprite: Sprite {
                        texture: texture.texture(),
                        size: vec2(2.0, 2.0),
                    },
                    time,
                    start_effect: action.as_ref().and_then(|(action_frame, action)| {
                        (frame == *action_frame).then(|| action.clone())
                    }),
                })
                .collect()
        },
    })
}
