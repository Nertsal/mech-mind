use super::*;

impl Logic<'_> {
    pub fn process_animations(&mut self) {
        self.process_units(Self::process_unit_animation);
    }

    fn process_unit_animation(&mut self, unit: &mut Unit) {
        unit.animation_state.frame_time += self.delta_time;
        let frame = unit
            .animation_state
            .animation
            .keyframes
            .get(unit.animation_state.frame)
            .expect("Failed to find animation frame");
        let delta = unit.animation_state.frame_time - frame.time;
        if delta >= Time::ZERO {
            // Next frame
            unit.animation_state.frame_time = delta;
            if unit.animation_state.animation.keyframes.len() <= unit.animation_state.frame + 1 {
                // Repeat
                unit.animation_state.frame = 0;
            } else {
                unit.animation_state.frame += 1;
            }
        }
    }
}
