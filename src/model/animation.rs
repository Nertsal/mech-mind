use super::*;

pub struct Animation {
    pub keyframes: Vec<AnimationFrame>,
}

pub struct AnimationFrame {
    pub sprite: Sprite,
    pub time: Time,
    /// Effect to perform at the start of the frame
    pub start_effect: Option<Effect>,
}

#[derive(Clone)]
pub struct AnimationState {
    pub animation: Rc<Animation>,
    pub frame: usize,
    pub frame_time: Time,
}

impl AnimationState {
    pub fn new(animation: &Rc<Animation>) -> (Self, Option<Effect>) {
        let state = Self {
            animation: animation.clone(),
            frame: 0,
            frame_time: Time::ZERO,
        };
        let effect = animation
            .keyframes
            .first()
            .expect("Animations cannot have zero frames")
            .start_effect
            .clone();
        (state, effect)
    }

    pub fn get_sprite(&self) -> &Sprite {
        &self.animation.keyframes.get(self.frame).unwrap().sprite
    }

    /// Update the animation state and returns whether it has started to loop
    /// and effects that should be processed.
    pub fn update(&mut self, delta_time: Time) -> (bool, Vec<Effect>) {
        self.frame_time += delta_time;

        let mut looped = false;
        let mut effects = Vec::new();

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
                    effects.push(effect);
                }
            } else {
                break;
            }
        }
        (looped, effects)
    }
}
