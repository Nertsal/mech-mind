use super::*;
use geng::Camera2d;

mod effect;
mod id;
mod collider;
mod sprite;
mod health;
mod animation;

pub use effect::*;
pub use animation::*;
pub use collider::*;
pub use health::*;
pub use id::*;
pub use sprite::*;

pub type Time = R32;
pub type Hp = R32;
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
    Engage(TargetAI),
    Idle,
}

pub enum TargetAI {
    Closest,
}

pub struct Action {
    pub cooldown: Time,
    pub engage_radius: Coord,
    pub animation: Rc<Animation>,
}

pub enum ActionState {
    Ready,
    InProgress { target: Option<Id> },
    Cooldown { time_left: Time },
}

#[derive(HasId)]
pub struct Unit {
    pub id: Id,
    pub faction: Faction,
    pub ai: UnitAI,
    pub health: Health,
    pub sanity: Option<Health>,
    pub collider: Collider,
    pub position: Position,
    pub velocity: Velocity,
    pub speed: Coord,
    pub acceleration: Coord,
    pub target_velocity: Velocity,
    pub action: Action,
    pub action_state: ActionState,
    pub animation_state: AnimationState,
}

#[derive(HasId)]
pub struct Projectile {
    pub id: Id,
    pub lifetime: Time,
    pub collider: Collider,
    pub on_hit: Effect,
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
