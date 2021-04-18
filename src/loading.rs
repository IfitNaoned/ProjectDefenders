use crate::app_state::*;
use bevy::prelude::*;
use bevy::{asset::LoadState, sprite::TextureAtlasBuilder};
use bevy_tilemap::prelude::*;

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

fn loading(
    mut commands: Commands,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut state: ResMut<State<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
) {
    println!("loading");
    if sprite_handles.atlas_loaded {
        return;
    }
    println!("loading 1");

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        println!("loading 1.1");
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
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

        println!("spawn tilemap_components");
        commands
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d());
        commands
            .spawn()
            .insert_bundle(tilemap_components)
            .insert(Timer::from_seconds(0.075, true));

        sprite_handles.atlas_loaded = true;
        state.set(AppState::StartMenu).unwrap();
    }
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    println!("load_textures");
    sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(load_textures.system().label("texture_loading")),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(loading.system())
                    .after("texture_loading"),
            );
    }
}
