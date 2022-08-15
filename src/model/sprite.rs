use super::*;

#[derive(Clone)]
pub struct Sprite {
    pub texture: Rc<ugli::Texture>,
    pub size: Vec2<f32>,
}

impl Sprite {
    pub fn new(texture: &Rc<ugli::Texture>, scale: f32) -> Self {
        Self {
            size: texture.size().map(|x| x as f32) * scale,
            texture: texture.clone(),
        }
    }
}

impl Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Sprite))
            .field("size", &self.size)
            .finish()
    }
}
