use super::*;

mod pixel_texture;

use geng::Sound;
pub use pixel_texture::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub background: BackgroundAssets,
    pub effects: EffectAssets,
    pub enemies: EnemyAssets,
    pub mech: MechAssets,
    pub ui: UIAssets,
    pub sound_design: SoundAssets,
}

#[derive(geng::Assets)]
pub struct SoundAssets {
    pub lava: Rc<Sound>,
    pub mechs: MechSounds,
    pub enemies: EnemySounds,
}

#[derive(geng::Assets)]
pub struct EnemySounds {
    #[asset(path = "death.mp3")]
    pub death: Rc<Sound>,
    pub blighter: BlighterSounds,
    pub ravager: RavagerSounds,
}

#[derive(geng::Assets)]
pub struct BlighterSounds {
    #[asset(path = "shoot.mp3")]
    pub shoot: Rc<Sound>,
    #[asset(path = "walk.mp3")]
    pub walk: Rc<Sound>,
}

#[derive(geng::Assets)]
pub struct RavagerSounds {
    #[asset(path = "roar.mp3")]
    pub roar: Rc<Sound>,
    #[asset(path = "walk.mp3")]
    pub walk: Rc<Sound>,
}

#[derive(geng::Assets)]
pub struct MechSounds {
    #[asset(path = "death.mp3")]
    pub death: Rc<Sound>,
    #[asset(path = "hit.mp3")]
    pub hit: Rc<Sound>,
    #[asset(path = "walk.mp3")]
    pub walk: Rc<Sound>,
    pub sanity_zero: Rc<Sound>,
    pub artillery: ArtillerySounds,
    pub tank: TankSounds,
    pub healer: HealerSounds,
}

#[derive(geng::Assets)]
pub struct TankSounds {
    #[asset(path = "shoot.mp3")]
    pub shoot: Rc<Sound>,
}

#[derive(geng::Assets)]
pub struct HealerSounds {
    pub heal_effect: Rc<Sound>,
}

#[derive(geng::Assets)]
pub struct ArtillerySounds {
    #[asset(path = "artillery_shoot.mp3")]
    pub artillery_shoot: Rc<Sound>,
    #[asset(path = "rocket_explode.mp3")]
    pub rocket_explode: Rc<Sound>,
}

#[derive(geng::Assets)]
pub struct UIAssets {
    pub enemy_health: PixelTexture,
    pub energy_bar: PixelTexture,
    pub mech_bar: PixelTexture,
    pub artillery_slot: PixelTexture,
    pub artillery_slot_bg: PixelTexture,
    pub healer_slot: PixelTexture,
    pub healer_slot_bg: PixelTexture,
    pub tank_slot: PixelTexture,
    pub tank_slot_bg: PixelTexture,
}

#[derive(geng::Assets)]
pub struct BackgroundAssets {
    pub background: PixelTexture,
    pub floor: PixelTexture,
    pub pillar1: PixelTexture,
    pub pillar2: PixelTexture,
    pub tower: PixelTexture,
    pub town: PixelTexture,
}

#[derive(geng::Assets)]
pub struct EffectAssets {
    #[asset(range = "1..=14", path = "heal/*.png")]
    pub heal: Vec<PixelTexture>,
}

#[derive(geng::Assets)]
pub struct EnemyAssets {
    pub blighter: BlighterEnemy,
    pub ravager: RavagerEnemy,
    pub stinger: StingerEnemy,
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
pub struct RavagerEnemy {
    #[asset(range = "1..=7", path = "anticipation/*.png")]
    pub anticipation: Vec<PixelTexture>,
    #[asset(range = "1..=5", path = "attack/*.png")]
    pub attack: Vec<PixelTexture>,
    #[asset(range = "1..=2", path = "charge/*.png")]
    pub charge: Vec<PixelTexture>,
    #[asset(range = "1..=20", path = "roar/*.png")]
    pub roar: Vec<PixelTexture>,
    #[asset(range = "1..=5", path = "walk/*.png")]
    pub walk: Vec<PixelTexture>,
    pub idle: PixelTexture,
}

#[derive(geng::Assets)]
pub struct StingerEnemy {
    #[asset(range = "1..=16", path = "attack/*.png")]
    pub attack: Vec<PixelTexture>,
    #[asset(range = "1..=5", path = "walk/*.png")]
    pub walk: Vec<PixelTexture>,
    pub idle: PixelTexture,
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
