use super::*;
use geng::Camera2d;

mod id;

pub use id::*;

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
    pub mechs: Collection<Mech>,
    pub enemies: Collection<Enemy>,
}

pub enum MechAI {
    Engage,
}

pub enum TargetAI {
    Closest,
}

#[derive(HasId)]
pub struct Mech {
    pub id: Id,
    pub position: Position,
    pub velocity: Velocity,
    pub size: Coord,
    pub ai: MechAI,
}

#[derive(HasId)]
pub struct Enemy {
    pub id: Id,
    pub position: Position,
    pub velocity: Velocity,
    pub size: Coord,
    pub target_ai: TargetAI,
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
            mechs: default(),
            enemies: default(),
        }
    }
}
