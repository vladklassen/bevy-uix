use bevy_asset::{AssetLoader, AsyncReadExt, LoadedAsset};
use bevy_scene::Scene;
use thiserror::Error;

use crate::{asset::UIX, parser};

pub struct UIXLoader;

impl AssetLoader for UIXLoader {
    type Asset = UIX;

    type Settings = ();

    type Error = UIXLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy_asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = String::with_capacity(1024);
        reader.read_to_string(&mut buf).await?;
        Ok(UIX {
            root: parser::parse(buf.as_str())?
        })
    }
    
    fn extensions(&self) -> &[&str] {
        &["ui.xml"]
    }
}

#[derive(Error, Debug)]
pub enum UIXLoaderError {
    #[error("Parse UIX error: {0}")]
    XmlError(#[from] parser::ParseError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}