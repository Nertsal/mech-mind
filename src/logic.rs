use super::*;
use model::*;
use std::collections::VecDeque;

mod action;
mod behaviour;
mod effects;
mod movement;

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
        self.process_actions();
        self.process_movement();
        self.process_effects();
    }
}
