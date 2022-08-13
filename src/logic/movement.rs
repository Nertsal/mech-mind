use super::*;

impl Logic<'_> {
    pub fn process_movement(&mut self) {
        for (position, velocity) in self
            .model
            .mechs
            .iter_mut()
            .map(|mech| (&mut mech.position, &mut mech.velocity))
            .chain(
                self.model
                    .enemies
                    .iter_mut()
                    .map(|enemy| (&mut enemy.position, &mut enemy.velocity)),
            )
        {
            *velocity += self.model.gravity * self.delta_time;
            *position += *velocity * self.delta_time;
            if position.y <= Coord::ZERO {
                // 0 is considered floor level
                position.y = Coord::ZERO;
                velocity.y = Coord::ZERO;
            }
        }
    }
}
