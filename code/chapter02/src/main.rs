use bevy::prelude::{
    App, AssetServer, Assets, ClearColor, Color, Commands, Component, DefaultPlugins, Input,
    KeyCode, Mut, OrthographicCameraBundle, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas,
    TextureAtlasSprite, Transform, Vec2, Vec3, WindowDescriptor, With,
};

use bevy::window::PresentMode;

const TILE_WIDTH: f32 = 22.;
const TILE_HEIGHT: f32 = 36.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct TilePosition {
    x: i32,
    y: i32,
    z: i32,
}

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

    // Spawn the player
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: TextureAtlasSprite::new(94),
            ..Default::default()
        })
        .insert(TilePosition { x: 0, y: 0, z: 0 });
}

fn tile_position_to_transform(tile_position: Mut<TilePosition>, mut transform: Mut<Transform>) {
    transform.translation = Vec3::new(
        tile_position.x as f32 * TILE_WIDTH,
        tile_position.y as f32 * TILE_HEIGHT,
        tile_position.z as f32,
    );
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut q: Query<(&mut Transform, &mut TilePosition), With<Player>>,
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
        tile_position_to_transform(tile_position, transform);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: 80.0 * TILE_WIDTH,
            height: 30.0 * TILE_HEIGHT,
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .run();
}
