use super::*;

impl Logic<'_> {
    pub fn process_behaviour(&mut self) {
        self.process_enemies();
        self.process_mechs();
    }

    fn process_enemies(&mut self) {
        for enemy in &mut self.model.enemies {
            match &enemy.target_ai {
                TargetAI::Closest => {
                    // TODO
                }
            }
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
