use super::*;

impl Logic<'_> {
    pub fn process_projectiles(&mut self) {
        // Move projectiles
        for projectile in &mut self.model.projectiles {
            projectile.velocity += self.model.gravity * self.delta_time;
            projectile.position += projectile.velocity * self.delta_time;
        }

        // Check for collisions
        for projectile in &mut self.model.projectiles {
            for unit in &self.model.units {
                if Some(unit.id) == projectile.caster {
                    continue;
                }
                if projectile
                    .collider
                    .check(&unit.collider, unit.position - projectile.position)
                {
                    projectile.lifetime = Time::ZERO;
                    self.effects.push_front(QueuedEffect {
                        effect: projectile.on_hit.clone(),
                        context: EffectContext {
                            caster: projectile.caster,
                            target: Some(unit.id),
                        },
                    });
                }
            }
        }

        // Remove collided projectiles
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }
}
