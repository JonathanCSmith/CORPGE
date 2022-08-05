// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use CORPGE::CORPGEPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CORPGEPlugin)
        .add_plugin(ActionsPlugin)
        .run();
}
