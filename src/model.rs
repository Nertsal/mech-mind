use super::*;
use geng::Camera2d;

mod effect;
mod id;
mod collider;
mod sprite;

pub use effect::*;
pub use collider::*;
pub use id::*;
pub use sprite::*;

pub type Time = R32;
pub type Coord = R32;
pub type Position = Vec2<Coord>;
pub type Velocity = Vec2<Coord>;

const GRAVITY: Vec2<f32> = vec2(0.0, -9.8);
const FOV: f32 = 20.0;

pub struct Model {
    pub id_gen: IdGen,
    pub gravity: Velocity,
    pub camera: Camera2d,
    pub units: Collection<Unit>,
    pub projectiles: Collection<Projectile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    Mech,
    Alien,
}

pub enum UnitAI {
    Mech(MechAI),
    Alien(TargetAI),
}

pub enum MechAI {
    Engage,
}

pub enum TargetAI {
    Closest,
}

pub struct Action {
    pub cooldown: Time,
    pub engage_radius: Coord,
    // TODO
    // pub animation: Animation,
    pub effect: Effect,
}

pub enum ActionState {
    Ready,
    InProgress { target: Option<Id> }, // TODO: Animation
    Cooldown { time_left: Time },
}

#[derive(HasId)]
pub struct Unit {
    pub id: Id,
    pub faction: Faction,
    pub ai: UnitAI,
    pub sprite: Sprite,
    pub collider: Collider,
    pub position: Position,
    pub velocity: Velocity,
    pub speed: Coord,
    pub acceleration: Coord,
    pub target_velocity: Velocity,
    pub action: Action,
    pub action_state: ActionState,
}

#[derive(HasId)]
pub struct Projectile {
    pub id: Id,
    pub collider: Collider,
    pub caster: Option<Id>,
    pub target: Option<Id>,
    pub position: Position,
    pub velocity: Velocity,
}

impl Model {
    pub fn new() -> Self {
        Self {
            id_gen: IdGen::new(),
            gravity: GRAVITY.map(Coord::new),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: FOV,
            },
            units: default(),
            projectiles: default(),
        }
    }
}
