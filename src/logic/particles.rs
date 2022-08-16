use super::*;

impl Logic<'_> {
    pub fn process_particles(&mut self) {
        for particle in &mut self.model.particles {
            if let Some(unit) = particle
                .follow_unit
                .and_then(|id| self.model.units.get(&id))
            {
                particle.position = unit.position;
            }
            let looped = particle.animation_state.update(self.delta_time);
            particle.alive = !looped;
            for effect in particle.animation_state.take_effects() {
                self.effects.push_front(QueuedEffect {
                    effect,
                    context: EffectContext {
                        caster: None,
                        target: particle.follow_unit,
                    },
                });
            }
        }
        self.model.particles.retain(|particle| particle.alive);
    }
}
