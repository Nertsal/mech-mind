use super::*;
use geng::Camera2d;

mod animation;
mod collider;
mod effect;
mod health;
mod id;
mod sprite;
mod unit_template;

pub use animation::*;
pub use collider::*;
pub use effect::*;
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
    pub templates: UnitTemplates,
    pub projectiles: Collection<Projectile>,
}

pub struct UnitTemplates {
    pub artillery: UnitTemplate,
    pub tank: UnitTemplate,
    pub healer: UnitTemplate,
    pub blighter: UnitTemplate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    Mech,
    Alien,
}

#[derive(Debug, Clone)]
pub enum UnitAI {
    Engage(TargetAI),
    Idle,
}

#[derive(Debug, Clone)]
pub enum TargetAI {
    Closest,
}

#[derive(Clone)]
pub struct Action {
    pub cooldown: Time,
    pub engage_radius: Coord,
    pub animation: Rc<Animation>,
}

#[derive(Debug, Clone)]
pub enum ActionState {
    Ready,
    InProgress { target: Option<Id> },
    Cooldown { time_left: Time },
}

#[derive(Clone)]
pub enum ExtraUnitRender {
    Tank {
        hand_pos: Position,
        weapon_pos: Position,
        /// Default is 0 degrees directed to the right
        rotation: Coord,
    }
}

#[derive(HasId, Clone)]
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
    pub flip_sprite: bool,
    pub animation_state: AnimationState,
    pub idle_animation: Rc<Animation>,
    pub extra_render: Option<ExtraUnitRender>,
}

#[derive(Clone)]
pub struct UnitTemplate {
    pub ai: UnitAI,
    pub health: Health,
    pub sanity: Option<Health>,
    pub collider: Collider,
    pub speed: Coord,
    pub acceleration: Coord,
    pub action: Action,
    pub idle_animation: Rc<Animation>,
    pub extra_render: Option<ExtraUnitRender>,
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
    pub fn new(assets: &Rc<Assets>) -> Self {
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
            templates: UnitTemplates::new(assets),
        }
    }
}
