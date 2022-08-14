use super::*;

impl Logic<'_> {
    pub fn process_actions(&mut self) {
        self.process_units(Self::process_unit_actions);
    }

    fn process_unit_actions(&mut self, unit: &mut Unit) {
        match &mut unit.action_state {
            ActionState::Ready => {}
            ActionState::InProgress { .. } => {} // Action effect is processed in the animation
            ActionState::Cooldown { time_left } => {
                *time_left -= self.delta_time;
                if *time_left <= Time::ZERO {
                    unit.action_state = ActionState::Ready;
                    if let UnitAI::Engage {
                        switch: Some(switch),
                        ..
                    } = &unit.ai
                    {
                        unit.action = switch.next_action.clone();
                        unit.ai = (*switch.next_ai).clone();
                    }
                }
            }
        }
    }
}
