use super::*;

impl Logic<'_> {
    pub fn process_animations(&mut self) {
        self.process_units(Self::process_unit_animation);
        for projectile in &mut self.model.projectiles {
            projectile.animation_state.update(self.delta_time);
        }
    }

    fn process_unit_animation(&mut self, unit: &mut Unit) {
        let looped = unit.animation_state.update(self.delta_time);
        let target = if let ActionState::InProgress { target } = unit.action_state {
            target
        } else {
            None
        };

        if looped {
            if let ActionState::InProgress { .. } = unit.action_state {
                // Stop action
                unit.action_state = ActionState::Cooldown {
                    time_left: unit.action.cooldown,
                };
                unit.animation_state.switch(&unit.idle_animation);
            }
        }

        for effect in unit.animation_state.take_effects() {
            let context = EffectContext {
                caster: Some(unit.id),
                target,
            };
            self.effects.push_front(QueuedEffect { effect, context });
        }
    }
}
