use crate::app_state::*;
use crate::map::*;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub fn loading(
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
    tilemap_atlas_handles: Res<TilemapAtlasHandles>,
    textures: Res<Assets<Texture>>,
) {
    let mut isloaded = false;
    asset_server.load::<Font, &'static str>("fonts/FiraMono-Medium.ttf");

    if asset_server.get_group_load_state(textures.iter().map(|(handle_id, _)| handle_id))
        == LoadState::Loaded
        && get_has_map_assets(asset_server, tilemap_atlas_handles)
    {
        isloaded = true;
    }

    if isloaded {
        println!("textures loaded");
        state.set(AppState::StartMenu).unwrap();
    }
}

pub fn load_textures(
    asset_server: Res<AssetServer>,
    mut tilemap_atlas_handles: ResMut<TilemapAtlasHandles>,
) {
    println!("load_textures");
    tilemap_atlas_handles.handles = asset_server.load_folder("textures").unwrap();
}

fn get_has_map_assets(
    asset_server: Res<AssetServer>,
    tilemap_atlas_handles: Res<TilemapAtlasHandles>,
) -> bool {
    asset_server.get_group_load_state(tilemap_atlas_handles.handles.iter().map(|handle| handle.id))
        == LoadState::Loaded
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TilemapAtlasHandles>()
            .add_system_set(
                SystemSet::on_enter(AppState::Loading).with_system(load_textures.system()),
            )
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading.system()));
    }
}
