use super::*;

#[derive(Debug, Clone)]
pub struct Animation {
    pub keyframes: Vec<AnimationFrame>,
}

#[derive(Debug, Clone)]
pub struct AnimationFrame {
    pub sprite: Sprite,
    pub time: Time,
    /// Effect to perform at the start of the frame
    pub start_effect: Option<Effect>,
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub animation: Rc<Animation>,
    pub frame: usize,
    pub frame_time: Time,
    pub effects: Vec<Effect>,
}

impl AnimationState {
    pub fn new(animation: &Rc<Animation>) -> Self {
        let mut state = Self {
            animation: animation.clone(),
            frame: 0,
            frame_time: Time::ZERO,
            effects: vec![],
        };
        let effect = animation
            .keyframes
            .first()
            .expect("Animations cannot have zero frames")
            .start_effect
            .clone();
        if let Some(effect) = effect {
            state.effects.push(effect);
        }
        state
    }

    pub fn switch(&mut self, animation: &Rc<Animation>) {
        if Rc::ptr_eq(&self.animation, animation) {
            // Same animation
            return;
        }
        *self = Self::new(animation);
    }

    pub fn get_sprite(&self) -> &Sprite {
        &self.animation.keyframes.get(self.frame).unwrap().sprite
    }

    pub fn take_effects(&mut self) -> Vec<Effect> {
        std::mem::take(&mut self.effects)
    }

    /// Update the animation state and returns whether it has started to loop.
    pub fn update(&mut self, delta_time: Time) -> bool {
        self.frame_time += delta_time;
        let mut looped = false;
        loop {
            let frame = self
                .animation
                .keyframes
                .get(self.frame)
                .expect("Failed to find animation frame");
            let delta = self.frame_time - frame.time;
            if delta >= Time::ZERO {
                // Next frame
                self.frame_time = delta;
                if self.animation.keyframes.len() <= self.frame + 1 {
                    // Repeat
                    looped = true;
                    self.frame = 0;
                } else {
                    self.frame += 1;
                }
                if let Some(effect) = self
                    .animation
                    .keyframes
                    .get(self.frame)
                    .unwrap()
                    .start_effect
                    .clone()
                {
                    self.effects.push(effect);
                }
            } else {
                break;
            }
        }
        looped
    }
}
