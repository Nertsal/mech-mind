use super::*;

impl Logic<'_> {
    pub fn process_waves(&mut self) {
        // Move the left border
        let min_pos = self
            .model
            .units
            .iter()
            .filter(|unit| unit.faction == Faction::Mech)
            .map(|unit| unit.position.x)
            .min();
        match min_pos {
            None => {
                // TODO: player lost
            }
            Some(min_pos) => {
                self.model.left_border = (min_pos - Coord::new(5.0)).max(self.model.left_border);
            }
        }

        // Check for waves
        if self.model.waves.is_empty() {
            self.generate_wave();
        }
        while let Some(wave) = self.model.waves.front() {
            if wave.position <= self.model.left_border {
                let wave = self.model.waves.pop_front().unwrap();
                for unit in wave.units {
                    let y = self.model.ground_level
                        + global_rng().gen_range(Coord::ZERO..=Coord::new(0.0));
                    let x = self.model.left_border
                        + global_rng().gen_range(Coord::new(50.0)..=Coord::new(70.0));
                    let position = vec2(x, y);
                    self.model.spawn_unit(unit, position, Faction::Alien);
                }
            } else {
                break;
            }
        }
    }

    fn get_difficulty(&self) -> R32 {
        let distance = self.model.left_border.max(R32::new(10.0));
        let team_size = self
            .model
            .units
            .iter()
            .filter(|unit| unit.faction == Faction::Mech)
            .count();
        let team_size = Coord::new(team_size as f32);

        distance + team_size
    }

    fn generate_wave(&mut self) {
        let mut difficulty = self.get_difficulty();
        let mut units = Vec::new();

        let templates = vec![
            (r32(5.0), &self.model.templates.blighter),
            (r32(3.0), &self.model.templates.ravager),
        ];
        let mut rng = global_rng();
        while let Some((diff, template)) = templates
            .iter()
            .filter(|(diff, _)| *diff <= difficulty)
            .choose(&mut rng)
        {
            difficulty -= *diff;
            units.push((*template).clone());
        }

        let position = self
            .model
            .waves
            .back()
            .map(|wave| wave.position)
            .unwrap_or(self.model.left_border)
            + Coord::new(30.0);
        let wave = Wave { position, units };
        self.model.waves.push_back(wave);
    }
}
