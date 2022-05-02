use std::collections::HashMap;

use bevy::prelude::{
    Commands, Component, Entity, Handle, Mut, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite,
    Transform, Vec3,
};

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};

const NUM_LAYERS: i32 = 5;

#[derive(Component, Clone)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32, z: i32) -> TilePosition {
        Self { x: x, y: y, z: z }
    }
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * TILE_WIDTH,
            self.y as f32 * TILE_HEIGHT,
            self.z as f32,
        )
    }
}

impl std::fmt::Display for TilePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Component)]
pub struct Wall;

pub fn tile_position_to_transform(tile_position: TilePosition, mut transform: Mut<Transform>) {
    transform.translation = tile_position.to_vec3();
}

pub struct Map {
    pub forward: Vec<Vec<Vec<Option<Entity>>>>,
    pub backward: HashMap<Entity, TilePosition>,
    pub width: usize,
    pub height: usize,
    pub num_layers: usize,
    pub zero_pos: TilePosition,
}
impl Map {
    pub fn new(width: usize, height: usize, zero_pos_x: i32, zero_pos_y: i32) -> Map {
        let num_layers = NUM_LAYERS as usize;
        Self {
            forward: vec![vec![vec![None; num_layers]; height]; width],
            backward: HashMap::new(),
            width: width,
            height: height,
            num_layers: num_layers,
            zero_pos: TilePosition {
                x: zero_pos_x,
                y: zero_pos_y,
                z: 0,
            },
        }
    }
    fn spawn_a_wall(
        &mut self,
        commands: &mut Commands,
        texture_atlas_handle: &Handle<TextureAtlas>,
        position: TilePosition,
    ) {
        let id = commands
            .spawn()
            .insert(Wall)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(position.to_vec3()),
                sprite: TextureAtlasSprite::new(186), // Full fill
                ..Default::default()
            })
            .insert(position.clone())
            .id();
        let corrected_x = position.x + self.zero_pos.x;
        let corrected_y = position.y + self.zero_pos.y;
        let corrected_z = position.z + self.zero_pos.z;
        self.forward[usize::try_from(corrected_x).ok().unwrap()]
            [usize::try_from(corrected_y).ok().unwrap()]
            [usize::try_from(corrected_z).ok().unwrap()] = Some(id.clone());
        self.backward.insert(id.clone(), position.clone());
    }
    pub fn spawn_walls(
        &mut self,
        commands: &mut Commands,
        texture_atlas_handle: &Handle<TextureAtlas>,
    ) {
        self.spawn_a_wall(commands, texture_atlas_handle, TilePosition::new(-1, 2, 0));
        self.spawn_a_wall(commands, texture_atlas_handle, TilePosition::new(0, 2, 0));
        self.spawn_a_wall(commands, texture_atlas_handle, TilePosition::new(1, 2, 0));
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Map width={} height={}>", self.width, self.height)
    }
}
