use super::*;

impl UnitTemplates {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            artillery: artillery(assets),
            tank: tank(assets),
            healer: healer(assets),
            blighter: blighter(assets),
            ravager: ravager(assets),
            stinger: stinger(assets),
        }
    }
}

impl UnitTemplate {
    pub fn instance(self, id_gen: &mut IdGen, position: Position, faction: Faction) -> Unit {
        Unit {
            id: id_gen.gen(),
            faction,
            ai: self.ai.clone(),
            health: self.health,
            sanity: self.sanity,
            collider: self.collider,
            position,
            velocity: Velocity::ZERO,
            speed: self.speed,
            acceleration: self.acceleration,
            target_velocity: Velocity::ZERO,
            statuses: default(),
            action: self.action,
            action_state: self.start_action_state,
            flip_sprite: false,
            animation_state: AnimationState::new(&self.idle_animation),
            idle_animation: self.idle_animation,
            move_animation: self.move_animation,
            extra_render: self.extra_render,
            on_death: self.on_death,
        }
    }
}

pub fn to_animation(
    textures: &[assets::PixelTexture],
    sprite_scale: f32,
    cycle_time: Time,
    actions: Vec<(usize, Effect)>,
) -> Rc<Animation> {
    let time = cycle_time / Time::new(textures.len() as f32);
    Rc::new(Animation {
        keyframes: {
            textures
                .iter()
                .enumerate()
                .map(|(frame, texture)| AnimationFrame {
                    sprite: Sprite::new(&texture.texture(), sprite_scale),
                    time,
                    start_effect: actions.iter().find_map(|(action_frame, action)| {
                        (frame == *action_frame).then(|| action.clone())
                    }),
                })
                .collect()
        },
    })
}

fn tank(assets: &Rc<Assets>) -> UnitTemplate {
    let idle_animation = to_animation(
        &[assets.mech.tank.idle.clone()],
        1.0 / 32.0,
        Time::ONE,
        vec![],
    );
    let move_animation = to_animation(&assets.mech.tank.walk, 1.0 / 32.0, Time::ONE, vec![]);
    let animation = to_animation(
        &assets.mech.tank.attack,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            2,
            Effect::List(Box::new(ListEffect {
                effects: vec![
                    Effect::Projectile(Box::new(ProjectileEffect {
                        offset: Position::ZERO,
                        ai: ProjectileAI::Idle,
                        collider: Collider::Aabb {
                            size: vec2(0.5, 0.5).map(Coord::new),
                        },
                        speed: Coord::new(10.0),
                        on_hit: Effect::Damage(Box::new(DamageEffect {
                            damage_type: DamageType::Physical,
                            value: Hp::new(1.0),
                        })),
                        animation: to_animation(
                            &[assets.mech.tank.projectile.clone()],
                            1.0 / 16.0,
                            Time::ONE,
                            vec![],
                        ),
                    })),
                    Effect::Sound(Box::new(SoundEffect {
                        sound: assets.sound_design.mechs.tank.shoot.clone(),
                    })),
                ],
            })),
        )],
    );
    UnitTemplate {
        ai: UnitAI::Engage {
            target: TargetAI::Closest,
            default: PositionAI::Advance,
            switch: None,
        },
        health: Health::new(Hp::new(30.0)),
        sanity: Some(Health::new(Hp::new(100.0))),
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(10.0),
        start_action_state: ActionState::Cooldown {
            time_left: Time::new(3.0),
        },
        action: Action {
            cooldown: Time::new(1.0),
            engage_radius: Coord::new(10.0),
            animation,
        },
        idle_animation,
        move_animation,
        extra_render: Some(ExtraUnitRender::Tank {
            hand_pos: vec2(-0.4, 0.5).map(Coord::new),
            weapon_pos: vec2(1.2, 0.1).map(Coord::new),
            shoot_pos: vec2(0.5, 0.0).map(Coord::new),
            rotation: Coord::ZERO,
        }),
        on_death: Effect::Noop,
    }
}

fn artillery(assets: &Rc<Assets>) -> UnitTemplate {
    let idle_animation = to_animation(
        &[assets.mech.artillery.idle.clone()],
        1.0 / 32.0,
        Time::ONE,
        vec![],
    );
    let move_animation = to_animation(&assets.mech.artillery.walk, 1.0 / 32.0, Time::ONE, vec![]);
    let animation = to_animation(
        &assets.mech.artillery.attack,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            2,
            Effect::List(Box::new(ListEffect {
                effects: vec![
                    Effect::Projectile(Box::new(ProjectileEffect {
                        offset: vec2(-0.5, 0.5).map(Coord::new),
                        ai: ProjectileAI::Rocket {
                            speed: Coord::new(15.0),
                            acceleration: Coord::new(20.0),
                            preferred_height: Coord::new(7.0),
                        },
                        collider: Collider::Aabb {
                            size: vec2(0.5, 0.5).map(Coord::new),
                        },
                        speed: Coord::ZERO,
                        on_hit: Effect::List(Box::new(ListEffect {
                            effects: vec![
                                Effect::Damage(Box::new(DamageEffect {
                                    damage_type: DamageType::Explosive,
                                    value: Hp::new(3.0),
                                })),
                                Effect::Sound(Box::new(SoundEffect {
                                    sound: assets
                                        .sound_design
                                        .mechs
                                        .artillery
                                        .rocket_explode
                                        .clone(),
                                })),
                            ],
                        })),
                        animation: to_animation(
                            &assets.mech.artillery.projectile_anim,
                            1.0 / 24.0,
                            Time::ONE,
                            vec![],
                        ),
                    })),
                    Effect::Sound(Box::new(SoundEffect {
                        sound: assets.sound_design.mechs.artillery.artillery_shoot.clone(),
                    })),
                ],
            })),
        )],
    );
    UnitTemplate {
        ai: UnitAI::Engage {
            target: TargetAI::Closest,
            default: PositionAI::Advance,
            switch: None,
        },
        health: Health::new(Hp::new(10.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(10.0),
        start_action_state: ActionState::Cooldown {
            time_left: Time::new(3.0),
        },
        action: Action {
            cooldown: Time::new(5.0),
            engage_radius: Coord::new(20.0),
            animation,
        },
        idle_animation,
        move_animation,
        extra_render: None,
        on_death: Effect::Noop,
    }
}

fn healer(assets: &Rc<Assets>) -> UnitTemplate {
    let idle_animation = to_animation(
        &[assets.mech.healer.idle.clone()],
        1.0 / 32.0,
        Time::ONE,
        vec![],
    );
    let move_animation = to_animation(&assets.mech.healer.walk, 1.0 / 32.0, Time::ONE, vec![]);
    let animation = to_animation(
        &assets.mech.healer.heal,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            5,
            Effect::List(Box::new(ListEffect {
                effects: vec![
                    Effect::Heal(Box::new(HealEffect {
                        value: Hp::new(5.0),
                    })),
                    Effect::Sound(Box::new(SoundEffect {
                        sound: assets.sound_design.mechs.healer.heal_effect.clone(),
                    })),
                ],
            })),
        )],
    );
    UnitTemplate {
        ai: UnitAI::Engage {
            target: TargetAI::LowestHp,
            default: PositionAI::Follow,
            switch: None,
        },
        health: Health::new(Hp::new(7.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.5),
        acceleration: Coord::new(10.0),
        start_action_state: ActionState::Cooldown {
            time_left: Time::new(3.0),
        },
        action: Action {
            cooldown: Time::new(2.0),
            engage_radius: Coord::new(5.0),
            animation,
        },
        idle_animation,
        move_animation,
        extra_render: None,
        on_death: Effect::Noop,
    }
}

fn blighter(assets: &Rc<Assets>) -> UnitTemplate {
    let idle_animation = to_animation(
        &[assets.enemies.blighter.idle.clone()],
        1.0 / 32.0,
        Time::ONE,
        vec![],
    );
    let move_animation = to_animation(&assets.enemies.blighter.walk, 1.0 / 32.0, Time::ONE, vec![]);
    let animation = to_animation(
        &assets.enemies.blighter.attack,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            10,
            Effect::List(Box::new(ListEffect {
                effects: vec![
                    Effect::Projectile(Box::new(ProjectileEffect {
                        offset: vec2(0.0, 0.5).map(Coord::new),
                        ai: ProjectileAI::Idle,
                        collider: Collider::Aabb {
                            size: vec2(0.5, 0.5).map(Coord::new),
                        },
                        speed: Coord::new(20.0),
                        on_hit: Effect::Damage(Box::new(DamageEffect {
                            damage_type: DamageType::Physical,
                            value: Hp::new(2.0),
                        })),
                        animation: to_animation(
                            &[assets.enemies.blighter.projectile.clone()],
                            1.0 / 32.0,
                            Time::ONE,
                            vec![],
                        ),
                    })),
                    Effect::Sound(Box::new(SoundEffect {
                        sound: assets.sound_design.enemies.blighter.shoot.clone(),
                    })),
                ],
            })),
        )],
    );
    UnitTemplate {
        ai: UnitAI::Engage {
            target: TargetAI::Farthest,
            default: PositionAI::Advance,
            switch: None,
        },
        health: Health::new(Hp::new(10.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(10.0),
        start_action_state: ActionState::Cooldown {
            time_left: Time::new(3.0),
        },
        action: Action {
            cooldown: Time::new(1.0),
            engage_radius: Coord::new(21.0),
            animation,
        },
        idle_animation,
        move_animation,
        extra_render: None,
        on_death: Effect::Noop,
    }
}

fn ravager(assets: &Rc<Assets>) -> UnitTemplate {
    let idle_animation = to_animation(
        &[assets.enemies.ravager.idle.clone()],
        1.0 / 32.0,
        Time::ONE,
        vec![],
    );
    let move_animation = to_animation(&assets.enemies.ravager.walk, 1.0 / 32.0, Time::ONE, vec![]);
    let roar = to_animation(
        &assets.enemies.ravager.roar,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            1,
            Effect::Sound(Box::new(SoundEffect {
                sound: assets.sound_design.enemies.ravager.roar.clone(),
            })),
        )],
    );
    let anticipation = to_animation(
        &assets.enemies.ravager.anticipation,
        1.0 / 32.0,
        Time::ONE,
        vec![],
    );
    let charge = to_animation(
        &assets.enemies.ravager.charge,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            1,
            Effect::List(Box::new(ListEffect {
                effects: vec![
                    Effect::Dash(Box::new(DashEffect {
                        speed: Coord::new(15.0),
                        duration: Time::new(0.5),
                        on_contact: Effect::Damage(Box::new(DamageEffect {
                            damage_type: DamageType::Physical,
                            value: Hp::new(5.0),
                        })),
                    })),
                    Effect::Sound(Box::new(SoundEffect {
                        sound: assets.sound_design.enemies.ravager.charge.clone(),
                    })),
                ],
            })),
        )],
    );
    let attack = to_animation(
        &assets.enemies.ravager.attack,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            3,
            Effect::List(Box::new(ListEffect {
                effects: vec![
                    Effect::Damage(Box::new(DamageEffect {
                        damage_type: DamageType::Physical,
                        value: Hp::new(1.0),
                    })),
                    Effect::Sound(Box::new(SoundEffect {
                        sound: assets.sound_design.enemies.ravager.bite.clone(),
                    })),
                ],
            })),
        )],
    );
    UnitTemplate {
        ai: UnitAI::Engage {
            target: TargetAI::Closest,
            default: PositionAI::Advance,
            switch: Some(SwitchAction {
                next_action: Action {
                    cooldown: Time::ZERO,
                    engage_radius: Coord::new(10.0),
                    animation: anticipation,
                },
                next_ai: Box::new(UnitAI::Engage {
                    target: TargetAI::Closest,
                    default: PositionAI::Advance,
                    switch: Some(SwitchAction {
                        next_action: Action {
                            cooldown: Time::new(2.0),
                            engage_radius: Coord::new(10.0),
                            animation: charge,
                        },
                        next_ai: Box::new(UnitAI::Engage {
                            target: TargetAI::Closest,
                            default: PositionAI::Advance,
                            switch: Some(SwitchAction {
                                next_action: Action {
                                    cooldown: Time::new(1.0),
                                    engage_radius: Coord::new(3.0),
                                    animation: attack,
                                },
                                next_ai: Box::new(UnitAI::Engage {
                                    target: TargetAI::Closest,
                                    default: PositionAI::Advance,
                                    switch: None,
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        },
        health: Health::new(Hp::new(20.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(2.0, 1.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(20.0),
        start_action_state: ActionState::Cooldown {
            time_left: Time::new(3.0),
        },
        action: Action {
            cooldown: Time::ZERO,
            engage_radius: Coord::new(10.0),
            animation: roar,
        },
        idle_animation,
        move_animation,
        extra_render: None,
        on_death: Effect::Noop,
    }
}

fn stinger(assets: &Rc<Assets>) -> UnitTemplate {
    let idle_animation = to_animation(
        &[assets.enemies.stinger.idle.clone()],
        1.0 / 32.0,
        Time::ONE,
        vec![],
    );
    let move_animation = to_animation(&assets.enemies.stinger.walk, 1.0 / 32.0, Time::ONE, vec![]);
    let attack = to_animation(
        &assets.enemies.stinger.attack,
        1.0 / 32.0,
        Time::ONE,
        vec![(
            13,
            Effect::Damage(Box::new(DamageEffect {
                damage_type: DamageType::Physical,
                value: Hp::new(5.0),
            })),
        )],
    );
    UnitTemplate {
        ai: UnitAI::Stinger {
            target: TargetAI::Closest,
            preferred_height: Coord::new(7.0),
            preferred_right: false,
            charge_speed: Coord::new(30.0),
        },
        health: Health::new(Hp::new(5.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(15.0),
        acceleration: Coord::new(10.0),
        start_action_state: ActionState::Cooldown {
            time_left: Time::new(5.0),
        },
        action: Action {
            cooldown: Time::new(5.0),
            engage_radius: Coord::new(10.0),
            animation: attack,
        },
        idle_animation,
        move_animation,
        extra_render: None,
        on_death: Effect::Noop,
    }
}
