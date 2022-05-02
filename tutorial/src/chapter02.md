# Drawing the "@" symbol and moving around

## Showing an empty black window with Bevy

Let's change the code in `main.rs` to the following:
```rust, noplayground
use bevy::prelude::{App, ClearColor, Color, DefaultPlugins, WindowDescriptor};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: 80.0 * 10.0,
            height: 50.0 * 10.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .run();
}
```

And we get the following window:

![Roguelike02-01](img/Roguelike02-01.png)

This part: 
```rust, noplayground
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: 80.0 * 22.0,
            height: 50.0 * 36.0,
            ..Default::default()
        })
```
is responsible for the creation of the window. 22 is the width in pixels in our font and 36 is the height. 80 and 50 are the number of columns and lines we will have in our window.

This line is responsible for almost black color of the background:
```rust, noplayground
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
```

And this part:
```rust, noplayground
        .add_plugins(DefaultPlugins)
```
adds default plugins of Bevy to the game engine.

## Adding PNG font to resources

In this tutorial I used the font `DejaVu Sans Mono`. I found a PNG file of this font [here](https://allfont.ru/download/dejavu-sans-mono/). Then edited it a bit and got the PNG that I liked. You can download the font from `code/chapter02/assets/DejaVu Sans Mono22.png` in the repository of this tutorial.

To load the PNG font I added the following code to `main.rs`:

```rust, noplayground
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("DejaVu Sans Mono22.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(22.0, 36.0), 31, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
}
```

I also added the following line:
```rust, noplayground
        .add_startup_system(setup)
```

to the `main` function:
```rust, noplayground
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: 80.0 * 22.0,
            height: 30.0 * 36.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .run();
}
```

The code is running, but nothing has changed because nothing yes uses the texture atlas.

## Drawing the `@` symbol
Let's create the player Entity and assign the `@` symbol:

First we need to define the player component:
```rust, noplayground
#[derive(Component)]
pub struct Player;
```

Then we should add the camera and the creation of the player to the `setup()` function:
```rust, noplayground
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
        });
```

We need the camera to render our sprites.

The player entity is created with the `spawn()` command and then we add components to this entity. To identify that this is a player we add the `Player` component. To add a sprite we add a `SpriteSheetBundle`. 
- `texture_atlas` is the atlas with our font that will be used. 
- `transform` is the position of the sprite. 0,0,0 means that it is in the center of the screen.
- `sprite` is the sprite that we use from the sprite sheet. Number 94 is the position of the sprite in the sprite sheet (3 lines each have 31 symbols totaling 93 symbols. We start counting at 0 so 92 is the last symbol in line 3 and 93 is the first symbol in line 4. We need the second symbol so the number should be 94)

And after `cargo run` we get the following picture:
![Roguelike02-02](img/Roguelike02-02.png)

## Moving around

Let's add the position in terms of columns and lines of characters (tile position) to the player. To do this we add the following component:
```rust, noplayground
#[derive(Component)]
pub struct TilePosition {
    x: i32,
    y: i32,
    z: i32,
}
```

And also add the component to the player in the setup:
```rust, noplayground
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
```

This function will transform tile position to transform position:
```rust, noplayground
fn tile_position_to_transform(tile_position: Mut<TilePosition>, mut transform: Mut<Transform>) {
    transform.translation = Vec3::new(
        tile_position.x as f32 * TILE_WIDTH,
        tile_position.y as f32 * TILE_HEIGHT,
        tile_position.z as f32,
    );
}
```

This is a new system that will read the keyboard input:
```rust, noplayground
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
```

Let's also add the system to the app:
```rust, noplayground
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
```
Another thing that I changed in the code above is:
```rust, noplayground
present_mode: PresentMode::Immediate,
```
This make the movement more responsive and without any lag that was there previously.

## Changing the color of the sprite from sprite sheet

We can change the color by changing lines in the creation of the SpriteSheetBundle. We substitute the following line:
```rust, noplayground
            sprite: TextureAtlasSprite::new(94),
```
with those lines:
```rust, noplayground
            sprite: {
                let mut sprite = TextureAtlasSprite::new(94);
                sprite.color = Color::BLUE;
                sprite
            },
```
This code uses the fact that expressions in Rust allow to return values. So here we created the `TextureAtlasSprite` with `new()` and then changed the color property of the sprite. In the last line the sprite is returned back to the constructor of SpriteSheetBundle.

Here is how the `.insert_bundle()` looks like now:

```rust, noplayground
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: {
                let mut sprite = TextureAtlasSprite::new(94);
                sprite.color = Color::BLUE;
                sprite
            },
            ..Default::default()
        })
```

The final code could be found here:
[Chapter02 code](https://github.com/keiv-fly/rust_bevy_roguelike_tutorial/tree/main/code/chapter02)



The final code of `main.rs` looks like this:
```rust, noplayground
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
```