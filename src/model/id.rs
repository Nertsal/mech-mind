use geng::prelude::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Id(u64);

pub struct IdGen {
    next_id: Id,
}

impl IdGen {
    pub fn new() -> Self {
        Self { next_id: Id(0) }
    }

    pub fn gen(&mut self) -> Id {
        let id = self.next_id;
        self.next_id.0 += 1;
        id
    }
}

impl Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}
