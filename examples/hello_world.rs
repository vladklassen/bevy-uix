use bevy::{
    prelude::*,
    render::{RenderPlugin, settings::Backends},
};
use bevy_uix::{UIXHandle, UIXPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "./assets".to_string(),
                    watch_for_changes_override: Some(true),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: bevy::render::settings::RenderCreation::Automatic(
                        bevy::render::settings::WgpuSettings {
                            backends: Some(Backends::DX12),
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                }),
            UIXPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(UIXHandle(asset_server.load("test.uix.xml")));
    // commands.spawn((Text::new("OK"), TextColor::WHITE, TextFont::default()));
}
