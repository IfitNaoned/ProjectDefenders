use crate::app_state::*;
use crate::layers::*;
use crate::map::*;
use bevy::prelude::*;
use bevy::render::camera::RenderLayers;
use bevy_mod_picking::*;

#[derive(Default)]
struct SelectedTile {
    entity: Option<Entity>,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Tile)
        .insert(RenderLayers::layer(DEBUG_LAYER));
}

fn tile_selection(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_tile: ResMut<SelectedTile>,
    picking_camera_query: Query<&PickingCamera>,
    tiles_query: Query<&Tile>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((tile_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(_tile) = tiles_query.get(tile_entity) {
                selected_tile.entity = Some(tile_entity);
            }
        } else {
            selected_tile.entity = None;
        }
    }
}

fn on_tile_selection(
    selected_tile: Res<SelectedTile>,
    materials: Res<TileMaterials>,
    mut query: Query<(Entity, &mut Handle<StandardMaterial>)>,
) {
    if !selected_tile.is_changed() {
        return;
    }

    for (entity, mut material) in query.iter_mut() {
        if Some(entity) == selected_tile.entity {
            *material = materials.selected_color.clone()
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedTile>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_player.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Game).with_system(tile_selection.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game).with_system(on_tile_selection.system()),
            );
    }
}
