use super::*;

impl Logic<'_> {
    pub fn process_deaths(&mut self) {
        // Units
        let mut sounds = Vec::new();
        self.model.units.retain(|unit| {
            let alive = unit.health.is_alive();
            if !alive {
                self.model.player_energy.change(Hp::new(10.0));
                let sound = match unit.faction {
                    Faction::Mech => self.model.assets.sound_design.mechs.death.clone(),
                    Faction::Alien => self.model.assets.sound_design.enemies.death.clone(),
                };
                sounds.push(sound);
            }
            alive
        });
        for sound in sounds {
            self.model.play_sound(&sound);
        }

        // Projectiles
        for projectile in &mut self.model.projectiles {
            projectile.lifetime -= self.delta_time;
        }
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }
}
