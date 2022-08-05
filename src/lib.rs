mod loader;

use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu
}

pub struct CORPGEPlugin;

impl Plugin for CORPGEPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(GameState::Loading)
            .add_plugin(LoaderPlugin);

        #[cfg(debug_assertions)]
        {
            app
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}