use super::*;

pub struct QueuedEffect {
    pub effect: Effect,
    pub context: EffectContext,
}

pub struct EffectContext {
    pub caster: Option<Id>,
    pub target: Option<Id>,
}

impl Logic<'_> {
    pub fn process_effects(&mut self) {
        const MAX_ITERATIONS: usize = 1000;
        let mut iterations = 0;
        while let Some(QueuedEffect {
            effect,
            mut context,
        }) = self.effects.pop_front()
        {
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
