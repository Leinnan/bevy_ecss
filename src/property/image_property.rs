use crate::{
    prelude::BevyCssError,
    property::{Property, PropertyValues},
};
use bevy::{
    ecs::query::QueryItem,
    prelude::{AssetServer, Commands, Node, With},
};
use bevy::ui::UiImage;

/// Applies the `image-path` property on [`bevy::ui::UiImage`] texture property of all sections on matched [`bevy::ui::UiImage`] components.
#[derive(Default)]
pub struct ImageProperty;

impl Property for ImageProperty {
    type Cache = String;
    type Components = &'static mut UiImage;
    type Filters = With<Node>;

    fn name() -> &'static str {
        "image-path"
    }

    fn parse<'a>(values: &PropertyValues) -> Result<Self::Cache, BevyCssError> {
        if let Some(path) = values.string() {
            Ok(path)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        components.texture = asset_server.load(cache);
    }
}
