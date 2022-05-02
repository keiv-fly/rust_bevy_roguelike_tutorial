mod constants;
mod map;
mod player;

use bevy::prelude::{
    App, AssetServer, Assets, ClearColor, Color, Commands, DefaultPlugins, Input, KeyCode, Mut,
    OrthographicCameraBundle, Query, Res, ResMut, TextureAtlas, Transform, Vec2, Vec3,
    WindowDescriptor, With,
};

use bevy::window::PresentMode;

use constants::{TILE_HEIGHT, TILE_WIDTH, WINDOW_COLUMNS, WINDOW_ROWS};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("DejaVu Sans Mono22.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_WIDTH, TILE_HEIGHT), 31, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add a 2D Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //Create the map
    let mut my_map = map::Map::new(
        usize::try_from(WINDOW_COLUMNS).ok().unwrap(),
        usize::try_from(WINDOW_ROWS).ok().unwrap(),
        WINDOW_COLUMNS / 2,
        WINDOW_ROWS / 2,
    );
    my_map.spawn_walls(&mut commands, &texture_atlas_handle);

    // Spawn the player
    player::spawn_player(&mut commands, texture_atlas_handle);

    // Insert the map as a resourse
    commands.insert_resource(my_map);
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut q: Query<(&mut Transform, &mut map::TilePosition), With<player::Player>>,
) {
    let mut tile_position_changed = false;
    let (transform, mut tile_position) = q.single_mut();
    if keys.just_pressed(KeyCode::Up) {
        tile_position.y += 1;
        tile_position_changed = true;
    } else if keys.just_pressed(KeyCode::Down) {
        tile_position.y -= 1;
        tile_position_changed = true;
    } else if keys.just_pressed(KeyCode::Left) {
        tile_position.x -= 1;
        tile_position_changed = true;
    } else if keys.just_pressed(KeyCode::Right) {
        tile_position.x += 1;
        tile_position_changed = true;
    }
    if tile_position_changed {
        map::tile_position_to_transform(tile_position.clone(), transform);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: WINDOW_COLUMNS as f32 * TILE_WIDTH,
            height: WINDOW_ROWS as f32 * TILE_HEIGHT,
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .run();
}
