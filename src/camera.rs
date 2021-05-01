use crate::app_state::*;
use bevy::prelude::*;
use bevy_tilemap::prelude::*;

fn camera(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tilemap_query: Query<&mut Tilemap>,
) {
    println!("camera!");
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    for mut map in tilemap_query.iter_mut() {
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

    // commands.spawn_bundle(Text2dBundle {
    //     text: Text::with_section(
    //         "This text is in the 2D scene.",
    //         TextStyle {
    //             font: asset_server.load::<Font, &'static str>("fonts/FiraMono-Medium.ttf"),
    //             font_size: 60.0,
    //             color: Color::WHITE,
    //         },
    //         TextAlignment {
    //             vertical: VerticalAlign::Center,
    //             horizontal: HorizontalAlign::Center,
    //         },
    //     ),
    //     ..Default::default()
    // });

    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..Default::default()
    // });

    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(camera.system()));
    }
}
