use super::*;

pub struct Background {
    objects: Vec<Sprite>,
    pub placed: Vec<(Position, Coord, Sprite)>,
    pub background: Repeating,
    pub floor: Repeating,
}

impl Background {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            placed: vec![],
            objects: vec![
                Sprite::new(&assets.background.pillar1, 0.03),
                Sprite::new(&assets.background.pillar2, 0.03),
                Sprite::new(&assets.background.tower, 0.03),
                Sprite::new(&assets.background.town, 0.03),
            ],
            background: Repeating::new(
                Sprite::new(
                    &assets.background.background,
                    FOV / 2.0 / assets.background.background.size().y as f32,
                ),
                Coord::new(1.0),
            ),
            floor: Repeating::new(
                Sprite::new(
                    &assets.background.floor,
                    FOV / 2.0 / assets.background.floor.size().y as f32,
                ),
                Coord::new(1.0),
            ),
        }
    }

    pub fn update(&mut self, max: Coord, delta_pos: Coord) {
        self.background.update(max, delta_pos);
        self.floor.update(max, delta_pos);

        // Move objects back
        for (pos, speed, _) in &mut self.placed {
            pos.x -= *speed * delta_pos;
        }

        // Remove objects that are out of view
        self.placed
            .retain(|(pos, _, sprite)| pos.x + Coord::new(sprite.size.x / 2.0) >= Coord::ZERO);

        // Place new objects
        let mut placed_max = self
            .placed
            .iter()
            .map(|(pos, _, sprite)| pos.x + Coord::new(sprite.size.x / 2.0))
            .max()
            .unwrap_or(Coord::ZERO);
        let mut rng = global_rng();
        while placed_max < max {
            match self.objects.choose(&mut rng) {
                None => break,
                Some(object) => {
                    let offset = rng.gen_range(Coord::new(0.5)..=Coord::new(2.0));
                    placed_max += offset + Coord::new(object.size.x);
                    let position = vec2(
                        placed_max - Coord::new(object.size.x / 2.0),
                        Coord::new(object.size.y / 2.0),
                    );
                    let speed = Coord::new(1.0);
                    self.placed.push((position, speed, object.clone()));
                }
            }
        }
    }
}
