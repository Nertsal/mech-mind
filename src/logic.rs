use super::*;
use model::*;
use std::collections::VecDeque;

mod action;
mod animation;
mod behaviour;
mod deaths;
mod effects;
mod movement;
mod projectiles;

pub use effects::*;

pub struct Logic<'a> {
    pub delta_time: Time,
    pub model: &'a mut Model,
    pub effects: VecDeque<QueuedEffect>,
}

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        let mut logic = Logic {
            delta_time,
            model: self,
            effects: default(),
        };
        logic.process();
    }
}

impl Logic<'_> {
    fn process(&mut self) {
        self.process_behaviour();
        self.process_animations();
        self.process_actions();
        self.process_movement();
        self.process_projectiles();
        self.process_effects();
        self.process_deaths();
    }

    fn process_units(&mut self, mut f: impl FnMut(&mut Self, &mut Unit)) {
        let ids: Vec<_> = self.model.units.ids().copied().collect();
        for id in ids {
            let mut unit = self.model.units.remove(&id).unwrap();
            f(self, &mut unit);
            self.model.units.insert(unit);
        }
    }
}
