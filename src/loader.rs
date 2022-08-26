use crate::GameState;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
//use bevy_kira_audio::AudioSource;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add in our assets (Config, DataPack)
        app
            .add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .continue_to_state(GameState::Menu)
            )
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(on_loading));
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

fn on_loading() {
    println!("Made it into loading!");
}
