use crate::app_state::*;
use crate::layers::*;
use bevy::prelude::*;
use bevy::render::{camera::RenderLayers, mesh::Indices, pipeline::PrimitiveTopology};
use bevy_mod_picking::*;

mod geometry;
mod hex;

pub struct Tile;

pub struct TileMaterials {
    pub selected_color: Handle<StandardMaterial>,
    pub grass_color: Handle<StandardMaterial>,
}
impl FromWorld for TileMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        TileMaterials {
            selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            grass_color: materials.add(Color::rgb(0.698, 0.941, 0.329).into()),
        }
    }
}

fn generate_map(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let height = 0.0;
    let mesh = meshes.add(generate_hex_mesh());

    for q in -5..5 {
        for r in -5..5 {
            let pos = geometry::center(1.0, &hex::HexCoord::new(q, r), &[0., height, 0.]);
            add_hex(
                Vec3::new(pos[0], pos[1], pos[2]),
                Color::rgb(0.698, 0.941, 0.329),
                mesh.clone(),
                &mut commands,
                &mut materials,
            );
        }
    }

    state.set(AppState::Game).unwrap();
}

fn add_hex(
    position: Vec3,
    color: Color,
    mesh: Handle<Mesh>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh,
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        // .insert_bundle(PickableBundle::default())
        .insert(Tile)
        .insert(RenderLayers::layer(DEBUG_LAYER));
}

fn generate_hex_mesh() -> Mesh {
    let mut pts: Vec<[f32; 3]> = vec![];
    let c = hex::HexCoord::new(0, 0);
    geometry::bevel_hexagon_points(&mut pts, 1.0, 0.9, &c);

    let mut normals: Vec<[f32; 3]> = vec![];
    geometry::bevel_hexagon_normals(&mut normals);

    let mut uvs: Vec<[f32; 2]> = vec![];
    for _ in 0..pts.len() {
        uvs.push([0., 0.]);
    }

    let mut indices = vec![];
    geometry::bevel_hexagon_indices(&mut indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, pts);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TileMaterials>().add_system_set(
            SystemSet::on_enter(AppState::Generating).with_system(generate_map.system()),
        );
    }
}
