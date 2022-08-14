use super::*;

impl Logic<'_> {
    pub fn process_behaviour(&mut self) {
        self.process_units(Self::process_unit_behaviour);
    }

    fn process_unit_behaviour(&mut self, unit: &mut Unit) {
        match &unit.ai {
            UnitAI::Idle => {}
            UnitAI::Engage(ai) => match ai {
                TargetAI::Closest => {
                    let target = self
                        .model
                        .units
                        .iter()
                        .filter(|other| other.faction != unit.faction)
                        .min_by_key(|enemy| (unit.position - enemy.position).len_sqr());
                    if let Some(target) = target {
                        let distance = (target.position - unit.position).len();
                        if distance > unit.action.engage_radius {
                            // Go towards the target
                            let vx = (target.position.x - unit.position.x).clamp_abs(unit.speed);
                            unit.target_velocity = vec2(vx, unit.velocity.y);
                            return;
                        } else if let ActionState::Ready = unit.action_state {
                            // The target is in range -> attack
                            unit.action_state = ActionState::InProgress {
                                target: Some(target.id),
                            };
                            let (state, effect) = AnimationState::new(&unit.action.animation);
                            unit.animation_state = state;
                            if let Some(effect) = effect {
                                self.effects.push_front(QueuedEffect {
                                    effect,
                                    context: EffectContext {
                                        caster: Some(unit.id),
                                        target: Some(target.id),
                                    },
                                })
                            }
                        }
                    }
                }
            },
        }
        unit.target_velocity = vec2(Coord::ZERO, unit.velocity.y);

        if let ActionState::InProgress { target } = &unit.action_state {
            if let Some(target_pos) = target
                .and_then(|id| self.model.units.get(&id))
                .map(|unit| unit.position)
            {
                if let Some(ExtraUnitRender::Tank {
                    hand_pos,
                    weapon_pos,
                    shoot_pos,
                    rotation,
                }) = &mut unit.extra_render
                {
                    // Aim at the target
                    if let Some(frame) = unit
                        .animation_state
                        .animation
                        .keyframes
                        .iter()
                        .skip(unit.animation_state.frame + 1)
                        .find(|frame| matches!(frame.start_effect, Some(Effect::Projectile(_))))
                    {
                        if let Some(Effect::Projectile(effect)) = &frame.start_effect {
                            let offset = *hand_pos + (*weapon_pos + *shoot_pos).rotate(*rotation);
                            if let Some((dir, _)) = aim_parabollically(
                                target_pos - (unit.position + offset),
                                self.model.gravity.y,
                                effect.speed,
                            ) {
                                let angle = dir.arg();
                                *rotation += (angle - *rotation)
                                    .clamp_abs(Coord::new(10.0) * self.delta_time);
                                // TODO: remove magic constant
                            }
                        }
                    }
                }
            }
        }
    }
}
