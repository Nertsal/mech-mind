use super::*;
use logic::*;

#[derive(Debug, Clone)]
pub enum Effect {
    Noop,
    Projectile(Box<ProjectileEffect>),
    Damage(Box<DamageEffect>),
}

#[derive(Debug, Clone)]
pub struct ProjectileEffect {
    pub offset: Position,
    pub on_hit: Effect,
}

#[derive(Debug, Clone)]
pub enum DamageType {
    Physical,
    Energy,
    Explosive,
}

#[derive(Debug, Clone)]
pub struct DamageEffect {
    pub damage_type: DamageType,
    pub value: Hp,
}

impl Effect {
    pub fn process(self, context: EffectContext, logic: &mut Logic) {
        match self {
            Effect::Noop => {}
            Effect::Projectile(effect) => effect.process(context, logic),
            Effect::Damage(effect) => effect.process(context, logic),
        }
    }
}

impl ProjectileEffect {
    pub fn process(self, context: EffectContext, logic: &mut Logic) {
        let caster = context.get_expect(Who::Caster, logic);
        let target = context.get_expect(Who::Target, logic);
        let position = self.offset + caster.position;
        let velocity = (target.position - caster.position).normalize_or_zero();
        logic.model.projectiles.insert(Projectile {
            id: logic.model.id_gen.gen(),
            lifetime: Time::new(10.0),
            collider: Collider::Aabb {
                size: vec2(1.0, 1.0).map(Coord::new),
            },
            on_hit: self.on_hit,
            caster: context.caster,
            target: context.target,
            position,
            velocity,
        });
    }
}

impl DamageEffect {
    pub fn process(self, context: EffectContext, logic: &mut Logic) {
        let target = context.get_mut_expect(Who::Target, logic);
        target.health.change(-self.value); // TODO: account for different damage types
    }
}
