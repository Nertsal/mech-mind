mod movement;
mod behaviour;

use super::*;

use model::*;

struct Logic<'a> {
    delta_time: Time,
    model: &'a mut Model,
}

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        let mut logic = Logic {
            delta_time,
            model: self,
        };
        logic.process();
    }
}

impl Logic<'_> {
    fn process(&mut self) {
        self.process_behaviour();
        self.process_movement();
    }
}
