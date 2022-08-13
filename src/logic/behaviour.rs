use super::*;

impl Logic<'_> {
    pub fn process_behaviour(&mut self) {
        self.process_enemies();
        self.process_mechs();
    }

    fn process_enemies(&mut self) {
        for enemy in &mut self.model.enemies {
            let target = match &enemy.target_ai {
                TargetAI::Closest => self
                    .model
                    .mechs
                    .iter()
                    .min_by_key(|mech| (mech.position - enemy.position).len_sqr()),
            };
            let vx = match target {
                Some(target) => (target.position.x - enemy.position.x).clamp_abs(enemy.speed),
                None => Coord::ZERO,
            };
            enemy.target_velocity = vec2(vx, enemy.velocity.y);
        }
    }

    fn process_mechs(&mut self) {
        for mech in &mut self.model.mechs {
            match &mech.ai {
                MechAI::Engage => {
                    let target = self
                        .model
                        .enemies
                        .iter()
                        .min_by_key(|enemy| (mech.position - enemy.position).len_sqr());
                    if let Some(target) = target {
                        let distance = (target.position - mech.position).len();
                        if distance > mech.action.engage_radius {
                            // Go towards the target
                            let vx = (target.position.x - mech.position.x).clamp_abs(mech.speed);
                            mech.target_velocity = vec2(vx, mech.velocity.y);
                            continue;
                        } else if let ActionState::Ready = mech.action_state {
                            // The target is in range -> attack
                            mech.action_state = ActionState::InProgress {
                                target: Some(target.id),
                            };
                        }
                    }
                }
            }
            mech.target_velocity = vec2(Coord::ZERO, mech.velocity.y);
        }
    }
}
