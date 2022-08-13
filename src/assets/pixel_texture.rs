use geng::prelude::*;

#[derive(Deref, DerefMut)]
pub struct PixelTexture(ugli::Texture);

impl ugli::Uniform for PixelTexture {
    fn apply(&self, gl: &ugli::raw::Context, info: &ugli::UniformInfo) {
        ugli::Texture::apply(self, gl, info)
    }
}

impl std::borrow::Borrow<ugli::Texture> for PixelTexture {
    fn borrow(&self) -> &ugli::Texture {
        &self.0
    }
}

impl std::borrow::Borrow<ugli::Texture> for &'_ PixelTexture {
    fn borrow(&self) -> &ugli::Texture {
        &self.0
    }
}

impl From<ugli::Texture> for PixelTexture {
    fn from(mut texture: ugli::Texture) -> Self {
        texture.set_filter(ugli::Filter::Nearest);
        Self(texture)
    }
}

impl geng::LoadAsset for PixelTexture {
    fn load(geng: &Geng, path: &std::path::Path) -> geng::AssetFuture<Self> {
        let texture = ugli::Texture::load(geng, path);
        async move { Ok(texture.await?.into()) }.boxed_local()
    }

    const DEFAULT_EXT: Option<&'static str> = Some("png");
}
