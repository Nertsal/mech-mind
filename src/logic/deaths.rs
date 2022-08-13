use super::*;

impl Logic<'_> {
    pub fn process_deaths(&mut self) {
        // Projectiles
        for projectile in &mut self.model.projectiles {
            projectile.lifetime -= self.delta_time;
        }
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }
}
