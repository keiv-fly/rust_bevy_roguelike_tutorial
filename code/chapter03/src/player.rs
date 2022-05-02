use bevy::prelude::{
    Color, Commands, Component, Handle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite,
    Transform, Vec3,
};

use crate::map;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: {
                let mut sprite = TextureAtlasSprite::new(94);
                sprite.color = Color::WHITE;
                sprite
            },
            ..Default::default()
        })
        .insert(map::TilePosition { x: 0, y: 0, z: 1 });
}
