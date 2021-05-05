use crate::app_state::*;
use crate::layers::*;
use bevy::prelude::*;
use bevy::render::{camera::RenderLayers, mesh::Indices, pipeline::PrimitiveTopology};
use bevy_mod_picking::*;

mod geometry;
mod hex;

pub static MAP_SIZE: isize = 21;

#[derive(Default, Debug)]
pub struct Tile {
    pub position: Vec3,
    pub is_center: bool,
    pub is_frontier: bool,
}

impl Tile {
    pub fn new(q: isize, r: isize) -> Self {
        let pos = geometry::center(
            1.0,
            &hex::HexCoord::new(q, r),
            &[-1.65 * (MAP_SIZE as f32) / 2., 0., -(MAP_SIZE as f32) / 2.],
        );

        let mut is_center = false;
        let mut is_frontier = false;

        if r == MAP_SIZE / 2 && q == MAP_SIZE / 2 {
            is_center = true;
        } else if r == 0 || q == 0 || r == MAP_SIZE || q == MAP_SIZE {
            is_frontier = true;
        }

        Self {
            position: Vec3::new(pos[0], pos[1], pos[2]),
            is_center,
            is_frontier,
        }
    }
}

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
    let mesh = meshes.add(generate_hex_mesh());

    for q in 0..MAP_SIZE {
        for r in 0..MAP_SIZE {
            add_hex(
                q,
                r,
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
    q: isize,
    r: isize,
    color: Color,
    mesh: Handle<Mesh>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let tile = Tile::new(q, r);

    commands
        .spawn_bundle(PbrBundle {
            mesh,
            material: materials.add(color.into()),
            transform: Transform::from_translation(tile.position),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(tile)
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

#[derive(Default)]
struct SelectedTile {
    entity: Option<Entity>,
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
            if let Ok(tile) = tiles_query.get(tile_entity) {
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

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TileMaterials>()
            .init_resource::<SelectedTile>()
            .add_system_set(
                SystemSet::on_enter(AppState::Generating).with_system(generate_map.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game).with_system(tile_selection.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game).with_system(on_tile_selection.system()),
            );
    }
}
