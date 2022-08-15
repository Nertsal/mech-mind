use geng::prelude::itertools::Itertools;

use super::*;

impl Logic<'_> {
    pub fn process_behaviour(&mut self) {
        self.process_units(Self::process_unit_behaviour);
    }

    fn process_unit_behaviour(&mut self, unit: &mut Unit) {
        match &unit.ai {
            UnitAI::Idle => {}
            UnitAI::Engage {
                target, default, ..
            } => {
                let target = find_target(unit, &self.model.units, target);
                if let Some(target) = target {
                    let distance = (target.position - unit.position).len();
                    if distance > unit.action.engage_radius {
                        // Go towards the target
                        let vx = (target.position.x - unit.position.x).clamp_abs(unit.speed);
                        unit.target_velocity = vec2(vx, unit.velocity.y);
                        if !Rc::ptr_eq(&unit.animation_state.animation, &unit.move_animation) {
                            let (state, effect) = AnimationState::new(&unit.move_animation);
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
                } else {
                    // Default
                    let target_dir = match default {
                        PositionAI::Advance => {
                            let friend = self
                                .model
                                .units
                                .iter()
                                .filter(|other| other.faction == unit.faction)
                                .map(|other| other.position.x)
                                .min();
                            if let Some(min) = friend {
                                if unit.position.x - min < Coord::new(7.0) {
                                    Coord::ONE
                                } else {
                                    Coord::ZERO
                                }
                            } else {
                                Coord::ONE
                            }
                        }
                        PositionAI::Follow => {
                            let friends = match self
                                .model
                                .units
                                .iter()
                                .filter(|other| other.faction == unit.faction)
                                .map(|other| other.position.x)
                                .minmax()
                            {
                                itertools::MinMaxResult::NoElements => None,
                                itertools::MinMaxResult::OneElement(x) => Some((x, x)),
                                itertools::MinMaxResult::MinMax(min, max) => Some((min, max)),
                            };
                            if let Some((min, max)) = friends {
                                let target = (min + max) / Coord::new(2.0);
                                target - unit.position.x
                            } else {
                                Coord::ONE
                            }
                        }
                    };
                    if target_dir != Coord::ZERO {
                        unit.target_velocity =
                            vec2(unit.speed * target_dir.signum(), unit.velocity.y);
                        if !Rc::ptr_eq(&unit.animation_state.animation, &unit.move_animation) {
                            let (state, effect) = AnimationState::new(&unit.move_animation);
                            unit.animation_state = state;
                            if let Some(effect) = effect {
                                self.effects.push_front(QueuedEffect {
                                    effect,
                                    context: EffectContext {
                                        caster: Some(unit.id),
                                        target: None,
                                    },
                                })
                            }
                        }
                        return;
                    }
                }
            }
            UnitAI::Stinger {
                target,
                preferred_height,
                preferred_right,
                charge_speed,
            } => {
                // Counteract gravity
                unit.velocity -= self.model.gravity * self.delta_time;
                match &unit.action_state {
                    ActionState::Ready => {
                        let preferred_distance = *charge_speed / Coord::new(4.0);
                        if let Some(target) = find_target(unit, &self.model.units, target) {
                            let delta = target.position - unit.position;
                            if delta.len() < preferred_distance {
                                // Start the attack animation
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
                            } else if !Rc::ptr_eq(
                                &unit.animation_state.animation,
                                &unit.move_animation,
                            ) {
                                let (state, effect) = AnimationState::new(&unit.move_animation);
                                unit.animation_state = state;
                                if let Some(effect) = effect {
                                    self.effects.push_front(QueuedEffect {
                                        effect,
                                        context: EffectContext {
                                            caster: Some(unit.id),
                                            target: None,
                                        },
                                    })
                                }
                            }
                            // Fly towards the target
                            unit.target_velocity = delta.normalize_or_zero() * *charge_speed;
                        }
                    }
                    ActionState::InProgress { target } => {
                        // Fly towards the target
                        if let Some(target) = target.and_then(|id| self.model.units.get(&id)) {
                            let delta = target.position - unit.position;
                            unit.target_velocity = delta.normalize_or_zero() * *charge_speed;
                        }
                    }
                    ActionState::Cooldown { .. } => {
                        // Fly around at the preferred height
                        if !Rc::ptr_eq(&unit.animation_state.animation, &unit.move_animation) {
                            let (state, effect) = AnimationState::new(&unit.move_animation);
                            unit.animation_state = state;
                            if let Some(effect) = effect {
                                self.effects.push_front(QueuedEffect {
                                    effect,
                                    context: EffectContext {
                                        caster: Some(unit.id),
                                        target: None,
                                    },
                                })
                            }
                        }
                        let bounds = AABB::points_bounding_box(
                            std::iter::once(vec2(self.model.left_border, Coord::ZERO)).chain(
                                self.model
                                    .units
                                    .iter()
                                    .filter(|unit| unit.faction == Faction::Mech)
                                    .map(|unit| unit.position),
                            ),
                        )
                        .extend_uniform(Coord::new(2.0));
                        let delta = if *preferred_right {
                            bounds.x_max - unit.position.x
                        } else {
                            unit.position.x
                                - bounds.x_min.max(self.model.left_border + Coord::new(2.0))
                        };
                        let preferred_right = if delta < Coord::ZERO {
                            !*preferred_right
                        } else {
                            *preferred_right
                        };
                        let preferred_height = *preferred_height;
                        unit.ai = UnitAI::Stinger {
                            target: target.clone(),
                            preferred_height,
                            preferred_right,
                            charge_speed: *charge_speed,
                        };
                        let vx = if preferred_right {
                            Coord::ONE
                        } else {
                            -Coord::ONE
                        };
                        let preferred_distance = unit.speed / Coord::new(2.0);
                        let y = (preferred_height - unit.position.y).clamp_abs(preferred_distance);
                        let x = (preferred_distance.sqr() - y * y).max(Coord::ZERO).sqrt() * vx;
                        unit.target_velocity = vec2(x, y);
                    }
                }
                return;
            }
        }
        unit.target_velocity = vec2(Coord::ZERO, unit.velocity.y);

        match &unit.action_state {
            ActionState::Ready | ActionState::Cooldown { .. } => {
                if !Rc::ptr_eq(&unit.animation_state.animation, &unit.idle_animation) {
                    unit.animation_state = AnimationState::new(&unit.idle_animation).0;
                }
            }
            ActionState::InProgress { target } => {
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
                        if let Some((index, frame)) = unit
                            .animation_state
                            .animation
                            .keyframes
                            .iter()
                            .skip(unit.animation_state.frame + 1)
                            .enumerate()
                            .find(|(_, frame)| {
                                matches!(frame.start_effect, Some(Effect::Projectile(_)))
                            })
                        {
                            if let Some(Effect::Projectile(effect)) = &frame.start_effect {
                                let mut offset =
                                    *hand_pos + (*weapon_pos + *shoot_pos).rotate(*rotation);
                                if unit.flip_sprite {
                                    offset.x = -offset.x;
                                }
                                let delta = target_pos - (unit.position + offset);
                                // Avoid awkward aim
                                if delta.len_sqr() > (*weapon_pos + *shoot_pos).len_sqr() {
                                    let dir = aim_parabollically(
                                        delta,
                                        self.model.gravity.y,
                                        effect.speed,
                                    )
                                    .map(|(dir, _)| dir)
                                    .unwrap_or(delta);
                                    let mut angle = dir.arg();
                                    if unit.flip_sprite {
                                        angle = Coord::PI - angle;
                                    }
                                    let time = unit
                                        .animation_state
                                        .animation
                                        .keyframes
                                        .get(unit.animation_state.frame)
                                        .unwrap()
                                        .time
                                        - unit.animation_state.frame_time
                                        + unit
                                            .animation_state
                                            .animation
                                            .keyframes
                                            .iter()
                                            .take(index)
                                            .skip(unit.animation_state.frame + 1)
                                            .map(|frame| frame.time)
                                            .fold(Time::ZERO, Time::add);
                                    let delta = angle - *rotation;
                                    *rotation += if time > Time::ZERO {
                                        delta.clamp_abs(delta / time * self.delta_time)
                                    } else {
                                        delta
                                    };
                                }
                            }
                        }
                    }

                    // Look at the target
                    unit.flip_sprite = (target_pos - unit.position).x < Coord::ZERO;
                }
            }
        }
    }
}

fn find_target<'a>(
    caster: &Unit,
    units: impl IntoIterator<Item = &'a Unit>,
    target: &TargetAI,
) -> Option<&'a Unit> {
    match target {
        TargetAI::Closest => units
            .into_iter()
            .filter(|other| other.faction != caster.faction)
            .min_by_key(|other| (caster.position - other.position).len_sqr()),
        TargetAI::Farthest => units
            .into_iter()
            .filter(|other| other.faction != caster.faction)
            .max_by_key(|other| (caster.position - other.position).len_sqr()),
        TargetAI::LowestHp => units
            .into_iter()
            .filter(|other| other.faction == caster.faction && other.health.ratio() < Hp::ONE)
            .min_by_key(|other| other.health.hp),
    }
}
