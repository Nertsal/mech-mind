use crate::game::PlayerEvent;

use super::*;

impl Model {
    pub fn handle_event(&mut self, event: PlayerEvent) {
        match event {
            PlayerEvent::SpawnMech { template } => {
                todo!()
            }
        }
    }
}
