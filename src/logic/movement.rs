use super::*;

impl Logic<'_> {
    pub fn process_movement(&mut self) {
        self.process_units(Self::process_unit_movement);
    }

    fn process_unit_movement(&mut self, unit: &mut Unit) {
        let acceleration = if unit
            .statuses
            .iter()
            .any(|status| matches!(status, Status::Charge { .. }))
        {
            Velocity::ZERO
        } else {
            (unit.target_velocity - unit.velocity).clamp_len(..=unit.acceleration * self.delta_time)
        };
        unit.velocity += acceleration + self.model.gravity * self.delta_time;
        unit.position += unit.velocity * self.delta_time;

        // Check ground
        if unit.position.y <= self.model.ground_level {
            unit.position.y = self.model.ground_level;
            unit.velocity.y = Coord::ZERO;
        }
        // Check left border
        if unit.position.x <= self.model.left_border {
            unit.position.x = self.model.left_border;
            unit.velocity.x = Coord::ZERO;
        }

        unit.flip_sprite = match unit.velocity.x.cmp(&Coord::ZERO) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => unit.flip_sprite,
            std::cmp::Ordering::Greater => false,
        };
    }
}
