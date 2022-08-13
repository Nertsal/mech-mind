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
                    self.model.units.insert(Unit {
                        id: self.model.id_gen.gen(),
                        faction: Faction::Mech,
                        ai: UnitAI::Mech(MechAI::Engage),
                        sprite: self.assets.mech.tank.idle.clone(),
                        position: vec2(0.0, 5.0).map(Coord::new),
                        velocity: Velocity::ZERO,
                        size: Coord::new(1.0),
                        speed: Coord::new(2.0),
                        acceleration: Coord::new(10.0),
                        target_velocity: Velocity::ZERO,
                        action: Action {
                            cooldown: Time::new(1.0),
                            engage_radius: Coord::new(2.0),
                            effect: Effect::Projectile(Box::new(ProjectileEffect {
                                offset: Position::ZERO,
                                on_hit: Effect::Noop,
                            })),
                        },
                        action_state: ActionState::Ready,
                    });
                }
                geng::Key::H => {
                    self.model.units.insert(Unit {
                        id: self.model.id_gen.gen(),
                        faction: Faction::Alien,
                        ai: UnitAI::Alien(TargetAI::Closest),
                        sprite: self.assets.mech.artillery.idle.clone(),
                        position: vec2(5.0, 5.0).map(Coord::new),
                        velocity: Velocity::ZERO,
                        size: Coord::new(1.0),
                        speed: Coord::new(2.0),
                        acceleration: Coord::new(10.0),
                        target_velocity: Velocity::ZERO,
                        action: Action {
                            cooldown: Time::new(1.0),
                            engage_radius: Coord::new(2.0),
                            effect: Effect::Noop,
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
