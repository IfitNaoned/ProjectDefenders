use crate::app_state::*;
use bevy::prelude::*;
use bevy_tilemap::prelude::*;

#[derive(Default, Clone)]
struct MapState {
    loaded: bool,
    spawned: bool,
}

fn generate_map(
    mut map_state: ResMut<MapState>,
    mut query: Query<&mut Tilemap>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    println!("generate_map!");

    if map_state.loaded {
        return;
    }

    for mut map in query.iter_mut() {
        println!("generate_map 1 !");
        let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
        let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;

        let grass_floor: Handle<Texture> = asset_server.get_handle("textures/hex-floor-grass.png");
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let grass_index = texture_atlas.get_texture_index(&grass_floor).unwrap();

        let mut tiles = Vec::new();
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let y = y - chunk_height / 2;
                let x = x - chunk_width / 2;
                let tile = Tile {
                    point: (x, y),
                    sprite_index: grass_index,
                    ..Default::default()
                };
                tiles.push(tile);
            }
        }
        map.insert_tiles(tiles).unwrap();

        map.spawn_chunk((-1, 0)).unwrap();
        map.spawn_chunk((0, 0)).unwrap();
        map.spawn_chunk((1, 0)).unwrap();
        map.spawn_chunk((-1, 1)).unwrap();
        map.spawn_chunk((0, 1)).unwrap();
        map.spawn_chunk((1, 1)).unwrap();
        map.spawn_chunk((-1, -1)).unwrap();
        map.spawn_chunk((0, -1)).unwrap();
        map.spawn_chunk((1, -1)).unwrap();
    }

    map_state.loaded = true;
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapState>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(generate_map.system()));
    }
}
