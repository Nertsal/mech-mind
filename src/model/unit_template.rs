use super::*;

impl UnitTemplates {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            artillery: artillery(assets),
            tank: tank(assets),
            healer: healer(assets),
            blighter: blighter(assets),
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
            action: self.action,
            action_state: ActionState::Ready,
            animation_state: AnimationState::new(&self.idle_animation).0,
            idle_animation: self.idle_animation,
            flip_sprite: false,
            extra_render: self.extra_render,
        }
    }
}

fn to_animation(
    textures: &[assets::PixelTexture],
    sprite_size: Vec2<f32>,
    cycle_time: Time,
    action: Option<(usize, Effect)>,
) -> Rc<Animation> {
    let time = cycle_time / Time::new(textures.len() as f32);
    Rc::new(Animation {
        keyframes: {
            textures
                .iter()
                .enumerate()
                .map(|(frame, texture)| AnimationFrame {
                    sprite: Sprite {
                        texture: texture.texture(),
                        size: sprite_size,
                    },
                    time,
                    start_effect: action.as_ref().and_then(|(action_frame, action)| {
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
        vec2(2.0, 2.0),
        Time::ONE,
        None,
    );
    let animation = to_animation(
        &assets.mech.tank.attack,
        vec2(2.0, 2.0),
        Time::ONE,
        Some((
            2,
            Effect::Projectile(Box::new(ProjectileEffect {
                offset: Position::ZERO,
                ai: ProjectileAI::Idle,
                speed: Coord::new(10.0),
                on_hit: Effect::Damage(Box::new(DamageEffect {
                    damage_type: DamageType::Physical,
                    value: Hp::new(1.0),
                })),
                animation: to_animation(
                    &[assets.mech.tank.projectile.clone()],
                    vec2(0.5, 0.2),
                    Time::ONE,
                    None,
                ),
            })),
        )),
    );
    UnitTemplate {
        ai: UnitAI::Engage(TargetAI::Closest),
        health: Health::new(Hp::new(10.0)),
        sanity: Some(Health::new(Hp::new(100.0))),
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(10.0),
        action: Action {
            cooldown: Time::new(1.0),
            engage_radius: Coord::new(10.0),
            animation,
        },
        idle_animation,
        extra_render: Some(ExtraUnitRender::Tank {
            hand_pos: vec2(-0.4, 0.5).map(Coord::new),
            weapon_pos: vec2(1.2, 0.1).map(Coord::new),
            shoot_pos: vec2(0.5, 0.0).map(Coord::new),
            rotation: Coord::ZERO,
        }),
    }
}

fn artillery(assets: &Rc<Assets>) -> UnitTemplate {
    let animation = to_animation(
        &assets.mech.artillery.attack,
        vec2(2.0, 2.0),
        Time::ONE,
        Some((
            2,
            Effect::Projectile(Box::new(ProjectileEffect {
                offset: vec2(-0.5, 0.5).map(Coord::new),
                ai: ProjectileAI::Rocket {
                    speed: Coord::new(15.0),
                    acceleration: Coord::new(20.0),
                    preferred_height: Coord::new(7.0),
                },
                speed: Coord::ZERO,
                on_hit: Effect::Damage(Box::new(DamageEffect {
                    damage_type: DamageType::Explosive,
                    value: Hp::new(3.0),
                })),
                animation: to_animation(
                    &assets.mech.artillery.projectile_anim,
                    vec2(1.0, 0.5),
                    Time::ONE,
                    None,
                ),
            })),
        )),
    );
    let idle_animation = to_animation(
        &[assets.mech.artillery.idle.clone()],
        vec2(2.0, 2.0),
        Time::ONE,
        None,
    );
    UnitTemplate {
        ai: UnitAI::Engage(TargetAI::Closest),
        health: Health::new(Hp::new(10.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(10.0),
        action: Action {
            cooldown: Time::new(5.0),
            engage_radius: Coord::new(20.0),
            animation,
        },
        idle_animation,
        extra_render: None,
    }
}

fn healer(assets: &Rc<Assets>) -> UnitTemplate {
    // let animation = to_animation(
    //     &assets.mech.healer.heal,
    //     vec2(2.0, 2.0),
    //     Time::ONE,
    //     None, // TODO: set action
    // );
    let idle_animation = to_animation(
        &[assets.mech.healer.idle.clone()],
        vec2(2.0, 2.0),
        Time::ONE,
        None,
    );
    UnitTemplate {
        ai: UnitAI::Idle,
        health: Health::new(Hp::new(10.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(10.0),
        action: Action {
            cooldown: Time::new(1.0),
            engage_radius: Coord::new(2.0),
            animation: idle_animation.clone(),
        },
        idle_animation,
        extra_render: None,
    }
}

fn blighter(assets: &Rc<Assets>) -> UnitTemplate {
    let animation = to_animation(
        &assets.enemies.blighter.attack,
        vec2(2.0, 2.0),
        Time::ONE,
        None, // TODO: set action
    );
    let idle_animation = to_animation(
        &[assets.enemies.blighter.idle.clone()],
        vec2(2.0, 2.0),
        Time::ONE,
        None,
    );
    UnitTemplate {
        ai: UnitAI::Idle,
        health: Health::new(Hp::new(10.0)),
        sanity: None,
        collider: Collider::Aabb {
            size: vec2(1.0, 2.0).map(Coord::new),
        },
        speed: Coord::new(2.0),
        acceleration: Coord::new(10.0),
        action: Action {
            cooldown: Time::new(1.0),
            engage_radius: Coord::new(2.0),
            animation,
        },
        idle_animation,
        extra_render: None,
    }
}
