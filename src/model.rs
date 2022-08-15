use std::collections::VecDeque;

use super::*;

mod animation;
mod collider;
mod effect;
mod health;
mod id;
mod sprite;
pub mod unit_template;
mod wave;

pub use animation::*;
pub use collider::*;
pub use effect::*;
pub use health::*;
pub use id::*;
pub use sprite::*;
pub use wave::*;

pub type Time = R32;
pub type Hp = R32;
pub type Coord = R32;
pub type Position = Vec2<Coord>;
pub type Velocity = Vec2<Coord>;

const GRAVITY: Vec2<f32> = vec2(0.0, -9.8);

pub struct Model {
    pub assets: Rc<Assets>,
    pub id_gen: IdGen,
    pub player_energy: Health,
    pub left_border: Coord,
    pub ground_level: Coord,
    pub gravity: Velocity,
    pub waves: VecDeque<Wave>,
    pub units: Collection<Unit>,
    pub templates: UnitTemplates,
    pub projectiles: Collection<Projectile>,
    pub particles: Collection<Particle>,
}

impl Model {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            assets: assets.clone(),
            id_gen: IdGen::new(),
            player_energy: Health {
                hp: Hp::ZERO,
                max_hp: Hp::new(100.0),
            },
            left_border: Coord::new(-20.0),
            ground_level: Coord::new(0.0),
            gravity: GRAVITY.map(Coord::new),
            waves: Wave::start_waves(),
            units: default(),
            templates: UnitTemplates::new(assets),
            projectiles: default(),
            particles: default(),
        }
    }
}

#[derive(HasId, Debug, Clone)]
pub struct Particle {
    pub id: Id,
    pub alive: bool,
    pub follow_unit: Option<Id>,
    pub position: Position,
    pub animation_state: AnimationState,
}

#[derive(Debug, Clone)]
pub enum Status {
    Charge { time: Time, on_contact: Effect },
}

pub struct UnitTemplates {
    pub artillery: UnitTemplate,
    pub tank: UnitTemplate,
    pub healer: UnitTemplate,
    pub blighter: UnitTemplate,
    pub ravager: UnitTemplate,
    pub stinger: UnitTemplate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    Mech,
    Alien,
}

#[derive(Debug, Clone)]
pub enum UnitAI {
    Idle,
    Stinger {
        target: TargetAI,
        preferred_height: Coord,
        preferred_right: bool,
        charge_speed: Coord,
    },
    Engage {
        target: TargetAI,
        default: PositionAI,
        switch: Option<SwitchAction>,
    },
}

#[derive(Debug, Clone)]
pub enum PositionAI {
    Advance,
    Follow,
}

#[derive(Debug, Clone)]
pub struct SwitchAction {
    pub next_action: Action,
    pub next_ai: Box<UnitAI>,
}

#[derive(Debug, Clone)]
pub enum TargetAI {
    Closest,
    Farthest,
    LowestHp,
}

#[derive(Debug, Clone)]
pub enum ProjectileAI {
    Idle,
    Rocket {
        speed: Coord,
        acceleration: Coord,
        preferred_height: Coord,
    },
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ExtraUnitRender {
    Tank {
        hand_pos: Position,
        weapon_pos: Position,
        shoot_pos: Position,
        /// Default is 0 degrees directed to the right
        rotation: Coord,
    },
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
    pub statuses: Vec<Status>,
    pub action: Action,
    pub action_state: ActionState,
    pub flip_sprite: bool,
    pub animation_state: AnimationState,
    pub idle_animation: Rc<Animation>,
    pub move_animation: Rc<Animation>,
    pub extra_render: Option<ExtraUnitRender>,
    pub on_death: Effect,
}

#[derive(Debug, Clone)]
pub struct UnitTemplate {
    pub ai: UnitAI,
    pub health: Health,
    pub sanity: Option<Health>,
    pub collider: Collider,
    pub speed: Coord,
    pub acceleration: Coord,
    pub start_action_state: ActionState,
    pub action: Action,
    pub idle_animation: Rc<Animation>,
    pub move_animation: Rc<Animation>,
    pub extra_render: Option<ExtraUnitRender>,
    pub on_death: Effect,
}

#[derive(HasId)]
pub struct Projectile {
    pub id: Id,
    pub ai: ProjectileAI,
    pub lifetime: Time,
    pub collider: Collider,
    pub on_hit: Effect,
    pub friend_faction: Option<Faction>,
    pub caster: Option<Id>,
    pub target: Option<Id>,
    pub position: Position,
    pub velocity: Velocity,
    pub animation_state: AnimationState,
}
