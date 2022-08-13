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
                    // TODO
                }
            }
        }
    }
}
