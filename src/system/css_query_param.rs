use super::query;

use crate::prelude::StyleSheetAsset;

use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(SystemParam)]
pub(crate) struct CssQueryParam<'w, 's> {
    pub assets: StyleSheetResource<'w>,
    pub ui_nodes: query::QueryUiNodes<'w, 's>,
    pub ui_changes: query::QueryUiChanges<'w, 's>,
    pub names: query::QueryEntityNames<'w, 's>,
    pub classes: query::QueryEntityClasses<'w, 's>,
    pub parent: query::QueryEntityParent<'w, 's>,
    pub children: query::QueryEntityChildren<'w, 's>,

    #[cfg(feature = "pseudo_class")]
    pub pseudo_classes: PseudoClassParam<'w, 's>,
}

#[derive(Deref, SystemParam)]
pub(crate) struct StyleSheetResource<'w>(Res<'w, Assets<StyleSheetAsset>>);

#[cfg(feature = "pseudo_class")]
#[derive(SystemParam)]
pub(crate) struct PseudoClassParam<'w, 's> {
    pub interaction: query::QueryEntityInteraction<'w, 's>,
    pub _children: query::QueryEntityChildren<'w, 's>,
}
