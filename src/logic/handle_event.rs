use crate::game::PlayerEvent;

use super::*;

impl Model {
    pub fn handle_event(&mut self, event: PlayerEvent) {
        match event {
            PlayerEvent::SpawnMech { template, cost } => {
                if self.player_energy.hp < cost {
                    return;
                }
                self.player_energy.change(-cost);
                let position = vec2(
                    self.left_border + Coord::new(5.0),
                    self.ground_level + Coord::new(5.0),
                );
                self.spawn_unit(template, position, Faction::Mech);
            }
        }
    }
}
