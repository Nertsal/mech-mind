use super::*;

impl Logic<'_> {
    pub fn process_projectiles(&mut self) {
        // Control behaviour
        for projectile in &mut self.model.projectiles {
            match &projectile.ai {
                ProjectileAI::Idle => {}
                ProjectileAI::Rocket {
                    speed,
                    acceleration,
                    preferred_height,
                } => {
                    let mut target_velocity = projectile.velocity.normalize_or_zero() * *speed;
                    if let Some(target) = projectile.target.and_then(|id| self.model.units.get(&id))
                    {
                        let preferred_distance = *speed / Coord::new(2.0);
                        let delta = target.position - projectile.position;
                        if delta.x.abs() < preferred_distance {
                            target_velocity = delta.normalize_or_zero() * *speed;
                        } else {
                            let y = (*preferred_height - projectile.position.y)
                                .clamp_abs(preferred_distance);
                            let x = (preferred_distance.sqr() - y * y).max(Coord::ZERO).sqrt()
                                * delta.x.signum();
                            target_velocity = vec2(x, y);
                        }
                    }
                    projectile.velocity += (target_velocity - projectile.velocity)
                        .clamp_len(..=*acceleration * self.delta_time);
                }
            }
        }

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
