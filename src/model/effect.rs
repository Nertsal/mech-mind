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
        info!("Projectile has spawned, pog!");
    }
}
