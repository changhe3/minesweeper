#![feature(rustc_attrs)]

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use tap::Tap;

mod components;
mod entities;
mod plugins;
mod resources;

fn main() {
    assert!(
        usize::BITS >= u32::BITS,
        "Only platforms with usize of 32 bits or more are supported"
    );

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Minesweeper".to_owned(),
            width: 1600.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .tap_mut(|app| {
            #[cfg(feature = "debug")]
            app.add_plugin(WorldInspectorPlugin::new());
        })
        .add_startup_system(camera_setup)
        .run();
}

fn camera_setup(mut cmds: Commands) {
    cmds.spawn_bundle(Camera2dBundle::default());
}
