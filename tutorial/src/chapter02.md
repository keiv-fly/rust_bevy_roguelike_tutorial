# Draw "@" symbol and moving around

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
            width: 80.0 * 10.0,
            height: 50.0 * 10.0,
            ..Default::default()
        })
```
is responsible for the creation of the window. 10 is the number of pixels in our font. 80 and 50 are the number of columns and lines we will have in our window.

This line is responsible for almost black color of the background:
```rust, noplayground
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
```

And this part:
```rust, noplayground
        .add_plugins(DefaultPlugins)
```
adds default plugins of Bevy to the game engine.

Let us now draw the "@" symbol.

## Drawing "@" symbol