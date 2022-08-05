use crate::GameState;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
//use bevy_kira_audio::AudioSource;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add in our assets (Config, DataPack)
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
        );
    }
}
