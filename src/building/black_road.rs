use crate::app_state::*;
use crate::layers::*;
use crate::map::*;
use bevy::prelude::*;
use bevy::render::camera::RenderLayers;
use rand::prelude::*;

pub struct BlackRoad;

fn spawn_black_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tiles_query: Query<&Tile>,
) {
    let black_road_tiles = tiles_query
        .iter()
        .filter(|t| t.is_frontier)
        .collect::<Vec<&Tile>>();

    for tile in black_road_tiles.choose_multiple(&mut rand::thread_rng(), 4) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                material: materials.add(Color::rgb(0., 0., 0.).into()),
                transform: Transform::from_translation(tile.position + Vec3::new(0., 0.01, 0.)),
                ..Default::default()
            })
            .insert(BlackRoad)
            .insert(RenderLayers::layer(GAME_ENTITY_LAYER));
    }
}

pub struct BlackRoadPlugin;
impl Plugin for BlackRoadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(spawn_black_road.system()),
        );
    }
}
