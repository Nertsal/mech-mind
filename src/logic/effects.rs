use super::*;

#[derive(Debug, Clone)]
pub struct QueuedEffect {
    pub effect: Effect,
    pub context: EffectContext,
}

#[derive(Debug, Clone)]
pub struct EffectContext {
    pub caster: Option<Id>,
    pub target: Option<Id>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Who {
    Caster,
    Target,
}

impl Logic<'_> {
    pub fn process_effects(&mut self) {
        const MAX_ITERATIONS: usize = 1000;
        let mut iterations = 0;
        while let Some(QueuedEffect { effect, context }) = self.effects.pop_front() {
            trace!("Processing {:?}", effect);
            effect.process(context, self);

            iterations += 1;
            if iterations > MAX_ITERATIONS {
                error!("Exceeded effect processing limit: {}", MAX_ITERATIONS);
                break;
            }
        }
    }
}

impl EffectContext {
    pub fn id(&self, who: Who) -> Option<Id> {
        match who {
            Who::Caster => self.caster,
            Who::Target => self.target,
        }
    }

    pub fn get<'a>(&self, who: Who, logic: &'a Logic<'_>) -> Option<&'a Unit> {
        self.id(who).and_then(|id| logic.model.units.get(&id))
    }

    pub fn id_expect(&self, who: Who) -> Id {
        self.id(who).unwrap_or_else(|| {
            panic!(
                "In the context {:?}, attempted to find {:?}, but they turned out to be None",
                self, who
            )
        })
    }

    pub fn get_expect<'a>(&self, who: Who, logic: &'a Logic<'_>) -> &'a Unit {
        let id = self.id_expect(who);
        logic.model.units.get(&id).unwrap_or_else(|| {
            panic!(
                "In the context {:?}, attempted to find {:?}, which have id {:?}, but they are present in the model",
                self, who, id
            )
        })
    }
}
