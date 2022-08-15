use super::*;

#[derive(Debug, Clone)]
pub struct Wave {
    pub position: Coord,
    pub units: Vec<UnitTemplate>,
}

impl Wave {
    pub fn start_waves() -> VecDeque<Self> {
        default()
    }
}
