use super::*;

impl Logic<'_> {
    pub fn process_behaviour(&mut self) {
        self.process_units(Self::process_unit_behaviour);
    }

    fn process_unit_behaviour(&mut self, unit: &mut Unit) {
        match &unit.ai {
            UnitAI::Idle => {}
            UnitAI::Engage(ai) => match ai {
                TargetAI::Closest => {
                    let target = self
                        .model
                        .units
                        .iter()
                        .filter(|other| other.faction != unit.faction)
                        .min_by_key(|enemy| (unit.position - enemy.position).len_sqr());
                    if let Some(target) = target {
                        let distance = (target.position - unit.position).len();
                        if distance > unit.action.engage_radius {
                            // Go towards the target
                            let vx = (target.position.x - unit.position.x).clamp_abs(unit.speed);
                            unit.target_velocity = vec2(vx, unit.velocity.y);
                            return;
                        } else if let ActionState::Ready = unit.action_state {
                            // The target is in range -> attack
                            unit.action_state = ActionState::InProgress {
                                target: Some(target.id),
                            };
                        }
                    }
                }
            },
        }
        unit.target_velocity = vec2(Coord::ZERO, unit.velocity.y);
    }
}
