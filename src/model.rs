use super::*;

mod id;

pub use id::*;

pub type Time = R32;

pub struct Model {
    pub id_gen: IdGen,
    pub mechs: Collection<Mech>,
}

#[derive(HasId)]
pub struct Mech {
    pub id: Id,
}

impl Model {
    pub fn new() -> Self {
        Self {
            id_gen: IdGen::new(),
            mechs: default(),
        }
    }
}
