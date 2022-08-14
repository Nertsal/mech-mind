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

    pub fn update(&mut self, delta_time: Time) -> Option<Effect> {
        self.frame_time += delta_time;
        let frame = self
            .animation
            .keyframes
            .get(self.frame)
            .expect("Failed to find animation frame");
        let delta = self.frame_time - frame.time;
        (delta >= Time::ZERO)
            .then(|| {
                // Next frame
                self.frame_time = delta;
                if self.animation.keyframes.len() <= self.frame + 1 {
                    // Repeat
                    self.frame = 0;
                } else {
                    self.frame += 1;
                }
                self.animation
                    .keyframes
                    .get(self.frame)
                    .unwrap()
                    .start_effect
                    .clone()
            })
            .flatten()
    }
}
