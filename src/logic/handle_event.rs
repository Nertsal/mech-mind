use crate::game::PlayerEvent;

use super::*;

impl Model {
    pub fn handle_event(&mut self, event: PlayerEvent) {
        match event {
            PlayerEvent::SpawnMech(mech) => {
                let (template, cost) = match mech {
                    game::MechType::Artillery => {
                        (self.templates.artillery.clone(), Currency::new(10.0))
                    }
                    game::MechType::Tank => (self.templates.tank.clone(), Currency::new(10.0)),
                    game::MechType::Healer => (self.templates.healer.clone(), Currency::new(10.0)),
                };
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
