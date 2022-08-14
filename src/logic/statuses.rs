use super::*;

impl Logic<'_> {
    pub fn process_statuses(&mut self) {
        self.process_units(Self::process_unit_statuses);
    }

    fn process_unit_statuses(&mut self, unit: &mut Unit) {
        for status in &mut unit.statuses {
            match status {
                Status::Charge { time, on_contact } => {
                    *time -= self.delta_time;
                    let effects: Vec<_> = self
                        .model
                        .units
                        .iter()
                        .filter(|other| {
                            other.faction != unit.faction
                                && other
                                    .collider
                                    .check(&unit.collider, unit.position - other.position)
                        })
                        .map(|other| QueuedEffect {
                            effect: on_contact.clone(),
                            context: EffectContext {
                                caster: Some(unit.id),
                                target: Some(other.id),
                            },
                        })
                        .collect();
                    for effect in effects {
                        self.effects.push_front(effect);
                    }
                }
            }
        }

        unit.statuses.retain(|status| match status {
            Status::Charge { time, .. } => *time > Time::ZERO,
        });
    }
}
