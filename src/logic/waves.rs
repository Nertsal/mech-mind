use super::*;

impl Logic<'_> {
    pub fn process_waves(&mut self) {
        // Move the left border
        let min_pos = self
            .model
            .units
            .iter()
            .filter(|unit| unit.faction == Faction::Mech)
            .map(|unit| unit.position.x)
            .min();
        match min_pos {
            None => {
                // TODO: player lost
            }
            Some(min_pos) => {
                self.model.left_border = (min_pos - Coord::new(5.0)).max(self.model.left_border);
            }
        }

        // Check for waves
        // TODO
    }
}
