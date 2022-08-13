use super::*;
use logic::*;

#[derive(Debug, Clone)]
pub enum Effect {
    Noop,
    Projectile(Box<ProjectileEffect>),
}

#[derive(Debug, Clone)]
pub struct ProjectileEffect {
    pub offset: Position,
    pub on_hit: Effect,
}

impl Effect {
    pub fn process(self, context: EffectContext, logic: &mut Logic) {
        match self {
            Effect::Noop => {}
            Effect::Projectile(effect) => effect.process(context, logic),
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
            caster: context.caster,
            target: context.target,
            position,
            velocity,
        });
    }
}
