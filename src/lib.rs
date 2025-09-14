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
                (handle_added_uix, spawn_uix).chain().before(bevy_ui::UiSystem::Content),
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

fn spawn_uix(
    mut commands: Commands,
    mut repository: ResMut<UIXRepository>,
    asset_server: Res<AssetServer>,
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
        
    }
}
