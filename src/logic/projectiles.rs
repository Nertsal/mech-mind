use super::*;

impl Logic<'_> {
    pub fn process_projectiles(&mut self) {
        // Move projectiles
        for projectile in &mut self.model.projectiles {
            projectile.position += projectile.velocity * self.delta_time;
        }

        // Check for collisions
        for projectile in &mut self.model.projectiles {
            // TODO
        }
    }
}
