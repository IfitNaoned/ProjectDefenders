use bevy::prelude::*;
use bevy_mod_picking::*;

mod app_state;
mod building;
mod camera;
mod layers;
mod loading;
mod map;
mod start_menu;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Project defenders".to_string(),
            width: 1024., //1920.,
            height: 720., //1080.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<PickingCamera>()
        .add_plugin(PickingPlugin)
        .add_plugin(app_state::AppStatePlugin)
        .add_plugin(loading::LoadingPlugin)
        .add_plugin(start_menu::StartMenuPlugin)
        .add_plugin(building::sacred_brazier::SacredBrazierPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(camera::CameraPlugin)
        .run();
}
