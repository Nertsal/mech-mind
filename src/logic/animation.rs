use super::*;

impl Logic<'_> {
    pub fn process_animations(&mut self) {
        self.process_units(Self::process_unit_animation);
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

        if looped && unit.animation_state.frame == 0 {
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
