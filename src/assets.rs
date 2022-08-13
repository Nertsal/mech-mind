use super::*;

mod pixel_texture;

pub use pixel_texture::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub mech: MechAssets,
}

#[derive(geng::Assets)]
pub struct MechAssets {
    pub artillery: ArtilleryMech,
    pub healer: HealerMech,
    pub tank: TankMech,
}

#[derive(geng::Assets)]
pub struct ArtilleryMech {
    #[asset(range = "1..=10", path = "attack/*.png")]
    pub attack: Vec<Rc<PixelTexture>>,
    #[asset(range = "1..=10", path = "walk/*.png")]
    pub walk: Vec<Rc<PixelTexture>>,
    pub idle: Rc<PixelTexture>,
}

#[derive(geng::Assets)]
pub struct HealerMech {
    #[asset(range = "1..=9", path = "heal/*.png")]
    pub heal: Vec<Rc<PixelTexture>>,
    #[asset(range = "1..=9", path = "walk/*.png")]
    pub walk: Vec<Rc<PixelTexture>>,
    pub idle: Rc<PixelTexture>,
}

#[derive(geng::Assets)]
pub struct TankMech {
    #[asset(range = "1..=3", path = "attack/*.png")]
    pub attack: Vec<Rc<PixelTexture>>,
    #[asset(range = "1..=10", path = "walk/*.png")]
    pub walk: Vec<Rc<PixelTexture>>,
    pub idle: Rc<PixelTexture>,
}

impl Assets {
    pub fn process(&mut self, _geng: &Geng) {}
}
