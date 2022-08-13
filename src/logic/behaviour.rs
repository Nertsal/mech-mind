use super::*;

impl Logic<'_> {
    pub fn process_behaviour(&mut self) {
        for mech in &mut self.model.mechs {
            match &mech.ai {
                MechAI::Engage => {
                    // TODO
                }
            }
        }
    }
}
