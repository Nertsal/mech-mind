use super::*;

pub struct Repeating {
    pub sprite: Sprite,
    speed: Coord,
    pub placed: VecDeque<Coord>,
}

impl Repeating {
    pub fn new(sprite: Sprite, speed: Coord) -> Self {
        Self {
            sprite,
            speed,
            placed: default(),
        }
    }

    pub fn update(&mut self, max: Coord, delta_pos: Coord) {
        // Move
        for coord in &mut self.placed {
            *coord -= self.speed * delta_pos;
        }

        let dx = Coord::new(self.sprite.size.x / 2.0);

        // Remove those that are out of view
        while let Some(coord) = self.placed.front() {
            if *coord + dx < Coord::ZERO {
                self.placed.pop_front();
            } else {
                break;
            }
        }

        // Add new ones
        let mut max_pos = self.placed.back().map(|x| *x + dx).unwrap_or(Coord::ZERO);
        while max_pos < max {
            self.placed.push_back(max_pos + dx);
            max_pos += dx + dx;
        }
    }
}
