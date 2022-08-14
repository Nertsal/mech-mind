use super::*;

impl Logic<'_> {
    pub fn process_animations(&mut self) {
        self.process_units(Self::process_unit_animation);
        for projectile in &mut self.model.projectiles {
            let (_, effects) = projectile.animation_state.update(self.delta_time);
            for effect in effects {
                let context = EffectContext {
                    caster: projectile.caster,
                    target: projectile.target,
                };
                self.effects.push_front(QueuedEffect { effect, context });
            }
        }
    }

    fn process_unit_animation(&mut self, unit: &mut Unit) {
        let (looped, effects) = unit.animation_state.update(self.delta_time);
        let target = if let ActionState::InProgress { target } = unit.action_state {
            target
        } else {
            None
        };
        for effect in effects {
            let context = EffectContext {
                caster: Some(unit.id),
                target,
            };
            self.effects.push_front(QueuedEffect { effect, context });
        }

        if looped {
            if let ActionState::InProgress { .. } = unit.action_state {
                // Stop action
                unit.action_state = ActionState::Cooldown {
                    time_left: unit.action.cooldown,
                };
                unit.animation_state = AnimationState::new(&unit.idle_animation).0;
            }
        }
    }
}
