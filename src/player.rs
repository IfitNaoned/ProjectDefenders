use crate::app_state::*;
use bevy::prelude::*;

fn spawn_player() {
    println!("spawn_player!");
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_player.system()));
    }
}
