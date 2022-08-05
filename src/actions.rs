use crate::GameState;

use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Actions>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(set_movement_actions)
            );
    }
}

#[derive!(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>
}