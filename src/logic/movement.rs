use super::*;

impl Logic<'_> {
    pub fn process_movement(&mut self) {
        self.process_units(Self::process_unit_movement);
    }

    fn process_unit_movement(&mut self, unit: &mut Unit) {
        let acceleration = (unit.target_velocity - unit.velocity)
            .clamp_len(..=unit.acceleration * self.delta_time);
        unit.velocity += acceleration + self.model.gravity * self.delta_time;
        unit.position += unit.velocity * self.delta_time;
        if unit.position.y <= Coord::ZERO {
            // 0 is considered floor level
            unit.position.y = Coord::ZERO;
            unit.velocity.y = Coord::ZERO;
        }
    }
}
