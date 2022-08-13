use super::*;

impl Logic<'_> {
    pub fn process_actions(&mut self) {
        self.process_units(Self::process_unit_actions);
    }

    fn process_unit_actions(&mut self, unit: &mut Unit) {
        match &mut unit.action_state {
            ActionState::Ready => {}
            ActionState::Cooldown { time_left } => {
                *time_left -= self.delta_time;
                if *time_left <= Time::ZERO {
                    unit.action_state = ActionState::Ready;
                }
            }
            ActionState::InProgress { target } => {
                self.effects.push_back(QueuedEffect {
                    effect: unit.action.effect.clone(),
                    context: EffectContext {
                        caster: Some(unit.id),
                        target: *target,
                    },
                });
                unit.action_state = ActionState::Cooldown {
                    time_left: unit.action.cooldown,
                };
            }
        }
    }
}
