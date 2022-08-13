use super::*;

pub struct Health {
    pub hp: Hp,
    pub max_hp: Hp,
}

impl Health {
    pub fn new(max_hp: Hp) -> Self {
        Self { hp: max_hp, max_hp }
    }

    pub fn change(&mut self, delta: Hp) {
        self.hp = (self.hp + delta).min(self.max_hp).max(Hp::ZERO);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > Hp::ZERO
    }
}
