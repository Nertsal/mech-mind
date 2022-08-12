use super::*;

impl Logic<'_> {
    pub fn process_movement(&mut self) {
        for mech in &mut self.model.mechs {
            mech.velocity += self.model.gravity * self.delta_time;
            mech.position += mech.velocity * self.delta_time;
            if mech.position.y <= Coord::ZERO {
                // 0 is considered floor level
                mech.position.y = Coord::ZERO;
                mech.velocity.y = Coord::ZERO;
            }
        }
    }
}
