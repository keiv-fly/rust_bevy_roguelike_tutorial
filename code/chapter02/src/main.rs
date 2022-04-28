use bevy::prelude::{
    App, AssetServer, Assets, ClearColor, Color, Commands, DefaultPlugins, Handle, Res, ResMut,
    TextureAtlas, Vec2, WindowDescriptor,
};

pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("terminal8x8.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1.0, 1.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // add sprite atlas as resource
    commands.insert_resource(CharsetAsset {
        atlas: texture_atlas_handle.clone(),
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: 80.0 * 10.0,
            height: 50.0 * 10.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .run();
}
