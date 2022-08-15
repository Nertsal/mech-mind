use crate::game::PlayerEvent;

use super::*;

impl Model {
    pub fn handle_event(&mut self, event: PlayerEvent) {
        match event {
            PlayerEvent::SpawnMech(mech) => {
                let (template, cost) = match mech {
                    game::MechType::Artillery => {
                        (self.templates.artillery.clone(), Currency::new(40.0))
                    }
                    game::MechType::Tank => (self.templates.tank.clone(), Currency::new(30.0)),
                    game::MechType::Healer => (self.templates.healer.clone(), Currency::new(25.0)),
                };
                if self.player_energy.hp < cost {
                    return;
                }
                self.player_energy.change(-cost);
                let mut rng = global_rng();
                let position = vec2(
                    self.left_border + Coord::new(rng.gen_range(2.0..=10.0)),
                    self.ground_level + Coord::new(rng.gen_range(2.0..=10.0)),
                );
                self.spawn_unit(template, position, Faction::Mech);
            }
        }
    }
}
