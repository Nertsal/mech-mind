use super::*;

impl Logic<'_> {
    pub fn process_deaths(&mut self) {
        // Units
        self.model.units.retain(|unit| {
            let alive = unit.health.is_alive();
            if !alive {
                self.model.player_energy.change(Hp::new(10.0));
            }
            alive
        });

        // Projectiles
        for projectile in &mut self.model.projectiles {
            projectile.lifetime -= self.delta_time;
        }
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }
}
