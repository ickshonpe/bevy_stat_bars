# Bevy Stat Bars

Simple library for drawing floating stat bars.

![](bars.png)

## About
* Successor to `bevy_simple_stat_bars`.
This has a much better design.
* Supports Bevy version 0.7
* The style options with this release are very limited, only colors and borders.
* Uses the builtin Bevy sprite renderer for drawing.

# Usage

In your Cargo.toml `[dependencies]` section, add the line:

```toml
bevy_stat_bars = "0.1"
```
This is a minimal app that should draw a 75% full stat bar in the middle of the window:
```rust
use bevy::prelude::*;
use bevy_stat_bars::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(StatBarsPlugin)
    .add_startup_system(|mut commands: Commands| { 
        commands
            .spawn_bundle(OrthographicCameraBundle::new_2d())
            .commands()
            .spawn_bundle((StatBar {
                value: 0.75,
                size: Vec2::new(200., 20.),
                ..Default::default()
            },));
    })   
    .run();
}
```

There are some complete examples you can run with the commands:
```
cargo run --example basic
cargo run --example interactive
```


# Notes
* The math is very hacky and I think there is a bug I think with the rotations where they don't quite compose in the natural way you'd expect. 

    It's not much of a problem though. Will be fixed next release.


