use bevy::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    Loading,
    StartMenu,
    Generating,
    Game,
}

pub struct AppStatePlugin;
impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Loading);
    }
}
