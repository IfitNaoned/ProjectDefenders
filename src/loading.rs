use crate::app_state::*;

use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct TexturesAtlasHandles {
    handles: Vec<HandleUntyped>,
}

pub fn loading(
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
    textures_atlas_handles: Res<TexturesAtlasHandles>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(
        textures_atlas_handles
            .handles
            .iter()
            .map(|handle| handle.id),
    ) {
        state.set(AppState::StartMenu).unwrap();
    }
}

pub fn load_textures(
    asset_server: Res<AssetServer>,
    mut tilemap_atlas_handles: ResMut<TexturesAtlasHandles>,
) {
    tilemap_atlas_handles.handles = asset_server.load_folder("textures").unwrap();
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TexturesAtlasHandles>()
            .add_system_set(
                SystemSet::on_enter(AppState::Loading).with_system(load_textures.system()),
            )
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading.system()));
    }
}
