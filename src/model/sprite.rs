use super::*;

#[derive(Clone)]
pub struct Sprite {
    pub texture: Rc<ugli::Texture>,
    pub size: Vec2<f32>,
}

impl Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Sprite))
            .field("size", &self.size)
            .finish()
    }
}
