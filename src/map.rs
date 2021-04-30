use crate::app_state::*;
use bevy::prelude::*;
use bevy_tilemap::prelude::*;

#[derive(Default, Clone)]
pub struct TilemapAtlasHandles {
    pub handles: Vec<HandleUntyped>,
}

pub fn spawn_map(
    mut commands: Commands,
    tilemap_atlas_handles: Res<TilemapAtlasHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    let tilemap_atlas_handle = tilemap_atlas_handles.handles[0].clone().typed::<Texture>();
    let atlas_texture = textures.get_mut(tilemap_atlas_handle.clone()).unwrap();
    atlas_texture.sampler.min_filter = bevy::render::texture::FilterMode::Nearest;
    atlas_texture.sampler.mag_filter = bevy::render::texture::FilterMode::Nearest;
    atlas_texture.sampler.mipmap_filter = bevy::render::texture::FilterMode::Nearest;

    let texture_atlas = TextureAtlas::from_grid(tilemap_atlas_handle, Vec2::new(32.0, 37.0), 6, 16);

    let atlas_handle = texture_atlases.add(texture_atlas);

    let tilemap = Tilemap::builder()
        .auto_chunk()
        .auto_spawn(2, 2)
        .topology(GridTopology::HexOddRows)
        .dimensions(3, 3)
        .chunk_dimensions(7, 4, 1)
        .texture_dimensions(32, 37)
        .texture_atlas(atlas_handle)
        .finish()
        .unwrap();

    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };

    commands
        .spawn()
        .insert_bundle(tilemap_components)
        .insert(AppState::Game);

    println!("map spawned");
}

pub fn generate_map(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<AppState>>,
    mut tilemap_query: Query<&mut Tilemap>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for mut map in tilemap_query.iter_mut() {
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

    println!("map generated");

    game_state.set(AppState::Game).unwrap();
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Generating)
                .with_system(spawn_map.system())
                .label("spawn_map"),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Generating)
                .with_system(generate_map.system())
                .after("spawn_map"),
        );
    }
}
