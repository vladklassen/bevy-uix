mod asset;
mod loader;
mod parser;

use bevy_app::prelude::*;
use bevy_asset::{AssetApp, AssetServer, Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_ecs::{
    query::Added,
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res},
};
use bevy_scene::{Scene, ScenePlugin, SceneRoot};
use bevy_text::prelude::*;
use bevy_ui::prelude::*;
use bevy_ui::widget::Text;

use crate::{asset::UIX, loader::UIXLoader};

pub struct UIXPlugin;

impl Plugin for UIXPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset::<UIX>()
            .register_asset_loader(UIXLoader)
            .init_resource::<UIXRepository>()
            .add_systems(
                PostUpdate,
                (handle_added_uix, spawn_new_uix)
                    .chain()
                    .before(bevy_ui::UiSystem::Content),
            );
    }
}

#[derive(Component)]
pub struct UIXHandle(pub Handle<UIX>);

#[derive(Resource, Default)]
struct UIXRepository {
    new_uix: Vec<Handle<UIX>>,
}

impl UIXRepository {
    fn add(&mut self, handle: Handle<UIX>) {
        self.new_uix.push(handle);
    }
}

fn handle_added_uix(
    mut repository: ResMut<UIXRepository>,
    new_uix: Query<&UIXHandle, Added<UIXHandle>>,
) {
    for uix_handle in new_uix.iter() {
        repository.add(uix_handle.0.clone());
    }
}

fn spawn_new_uix(
    mut commands: Commands,
    mut repository: ResMut<UIXRepository>,
    uix_assets: Res<Assets<UIX>>,
) {
    let mut i = 0;
    loop {
        if i >= repository.new_uix.len() {
            break;
        }
        let handle = repository.new_uix[i].clone();
        let Some(uix) = uix_assets.get(handle.id()) else {
            i += 1;
            continue;
        };
        repository.new_uix.swap_remove(i);
        spawn_uix(&mut commands, &uix.root);
    }
}

fn spawn_uix(commands: &mut Commands, uix_node: &crate::asset::Node) {
    let mut node = Node::DEFAULT;
    node.display = uix_node.display;
    node.flex_direction = uix_node.flex_direction;
    node.flex_wrap = uix_node.flex_wrap;
    node.flex_grow = uix_node.flex_grow;
    node.flex_shrink = uix_node.flex_shrink;
    node.flex_basis = uix_node.flex_basis;
    node.width = uix_node.width;
    node.height = uix_node.height;
    node.min_width = uix_node.min_width;
    node.min_height = uix_node.min_height;
    node.max_width = uix_node.max_width;
    node.max_height = uix_node.max_height;
    node.top = uix_node.top;
    node.bottom = uix_node.bottom;
    node.left = uix_node.left;
    node.right = uix_node.right;
    node.margin = uix_node.margin;
    node.padding = uix_node.padding;

    
}
