use super::*;

impl Logic<'_> {
    pub fn process_actions(&mut self) {
        for (caster, state, action) in self
            .model
            .mechs
            .iter_mut()
            .map(|mech| (mech.id, &mut mech.action_state, &mut mech.action))
        {
            match state {
                ActionState::Ready => {}
                ActionState::Cooldown { time_left } => {
                    *time_left -= self.delta_time;
                    if *time_left <= Time::ZERO {
                        *state = ActionState::Ready;
                    }
                }
                ActionState::InProgress { target } => {
                    self.effects.push_back(QueuedEffect {
                        effect: action.effect.clone(),
                        context: EffectContext {
                            caster: Some(caster),
                            target: *target,
                        },
                    });
                    *state = ActionState::Cooldown {
                        time_left: action.cooldown,
                    };
                }
            }
        }
    }
}
