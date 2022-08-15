use super::*;

impl Logic<'_> {
    pub fn process_coins(&mut self) {
        for coin in &mut self.model.coins {
            if self
                .model
                .units
                .iter()
                .filter(|unit| unit.faction == Faction::Mech)
                .any(|unit| {
                    coin.collider
                        .check(&unit.collider, unit.position - coin.position)
                })
            {
                coin.alive = false;
                self.model.player_coins += 1; // TODO: coin value
            }
        }

        self.model.coins.retain(|coin| coin.alive);
    }
}
