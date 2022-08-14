use super::*;

impl Logic<'_> {
    pub fn process_animations(&mut self) {
        self.process_units(Self::process_unit_animation);
    }

    fn process_unit_animation(&mut self, unit: &mut Unit) {
        if let Some(effect) = unit.animation_state.update(self.delta_time) {
            let mut context = EffectContext {
                caster: Some(unit.id),
                target: None,
            };
            if let ActionState::InProgress { target } = unit.action_state {
                context.target = target;
                if unit.animation_state.frame == 0 {
                    // Stop action
                    unit.action_state = ActionState::Cooldown {
                        time_left: unit.action.cooldown,
                    };
                }
            }
            self.effects.push_front(QueuedEffect { effect, context });
        }
    }
}
