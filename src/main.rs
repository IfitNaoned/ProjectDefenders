use bevy::prelude::*;
use bevy_tilemap::prelude::*;

mod app_state;
use app_state::*;

mod loading;
use loading::*;

mod start_menu;
use start_menu::*;

mod player;
use player::*;

mod map;
use map::*;

mod camera;
use camera::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Project defenders".to_string(),
            width: 1024., //1920.,
            height: 720., //1080.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_plugin(AppStatePlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(StartMenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .run();
}
