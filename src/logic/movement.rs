use super::*;

impl Logic<'_> {
    pub fn process_movement(&mut self) {
        for (position, velocity, acceleration) in self
            .model
            .mechs
            .iter_mut()
            .map(|mech| (&mut mech.position, &mut mech.velocity, Velocity::ZERO))
            .chain(self.model.enemies.iter_mut().map(|enemy| {
                let acceleration = (enemy.target_velocity - enemy.velocity)
                    .clamp_len(..=enemy.acceleration * self.delta_time);
                (&mut enemy.position, &mut enemy.velocity, acceleration)
            }))
        {
            *velocity += acceleration + self.model.gravity * self.delta_time;
            *position += *velocity * self.delta_time;
            if position.y <= Coord::ZERO {
                // 0 is considered floor level
                position.y = Coord::ZERO;
                velocity.y = Coord::ZERO;
            }
        }
    }
}
