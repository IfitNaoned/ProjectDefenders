use crate::app_state::*;
use crate::building::*;
use crate::layers::*;
use crate::map::*;
use bevy::prelude::*;
use bevy::render::camera::RenderLayers;
use bevy_mod_picking::*;

pub struct SacredBrazier;

fn spawn_sacred_brazier(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tiles_query: Query<&Tile>,
) {
    if let Some(tile) = tiles_query.iter().find(|tile| tile.is_center) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(tile.position + Vec3::new(0., 0.5, 0.)),
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(SacredBrazier)
            .insert(LifePoint { life_point: 100 })
            .insert(RenderLayers::layer(GAME_ENTITY_LAYER));
    }
}

pub struct SacredBrazierPlugin;
impl Plugin for SacredBrazierPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(spawn_sacred_brazier.system()),
        );
    }
}
