use super::StyleSheetAsset;
use bevy::asset::AsyncReadExt;
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use crate::stylesheet::style_sheet_loader::StyleSheetLoaderError;

#[derive(Default)]
pub(crate) struct SassStyleSheetLoader;

impl AssetLoader for SassStyleSheetLoader {
    type Asset = StyleSheetAsset;
    type Settings = ();
    type Error = StyleSheetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let content = std::str::from_utf8(&bytes)?;
            let parsed = match grass::from_string(content, &grass::Options::default()) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to parse styles. Error: {}", e);
                    "".to_string()
                }
            };
            let stylesheet =
                StyleSheetAsset::parse(load_context.path().to_str().unwrap_or_default(), &parsed);
            Ok(stylesheet)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sass"]
    }
}
