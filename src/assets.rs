use super::*;

mod pixel_texture;

pub use pixel_texture::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub enemies: EnemyAssets,
    pub mech: MechAssets,
}

#[derive(geng::Assets)]
pub struct EnemyAssets {
    pub blighter: BlighterEnemy,
}

#[derive(geng::Assets)]
pub struct MechAssets {
    pub artillery: ArtilleryMech,
    pub healer: HealerMech,
    pub tank: TankMech,
}

#[derive(geng::Assets)]
pub struct BlighterEnemy {
    #[asset(range = "1..=16", path = "attack/*.png")]
    pub attack: Vec<PixelTexture>,
    #[asset(range = "1..=10", path = "walk/*.png")]
    pub walk: Vec<PixelTexture>,
    pub idle: PixelTexture,
    pub projectile: PixelTexture,
}

#[derive(geng::Assets)]
pub struct ArtilleryMech {
    #[asset(range = "1..=10", path = "attack/*.png")]
    pub attack: Vec<PixelTexture>,
    #[asset(range = "1..=10", path = "walk/*.png")]
    pub walk: Vec<PixelTexture>,
    pub idle: PixelTexture,
    #[asset(range = "1..=6", path = "projectile_anim/*.png")]
    pub projectile_anim: Vec<PixelTexture>,
    pub projectile: PixelTexture,
}

#[derive(geng::Assets)]
pub struct HealerMech {
    #[asset(range = "1..=9", path = "heal/*.png")]
    pub heal: Vec<PixelTexture>,
    #[asset(range = "1..=9", path = "walk/*.png")]
    pub walk: Vec<PixelTexture>,
    pub idle: PixelTexture,
}

#[derive(geng::Assets)]
pub struct TankMech {
    #[asset(range = "1..=3", path = "attack/*.png")]
    pub attack: Vec<PixelTexture>,
    #[asset(range = "1..=10", path = "walk/*.png")]
    pub walk: Vec<PixelTexture>,
    pub idle: PixelTexture,
    pub hand: PixelTexture,
    pub weapon: PixelTexture,
    pub projectile: PixelTexture,
}

impl Assets {
    pub fn process(&mut self, _geng: &Geng) {}
}
