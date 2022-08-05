# Bevy Stat Bars

Simple library for drawing floating stat bars.

![](bars.png)

## About
* Successor to `bevy_simple_stat_bars`.
* version 0.2+ supports Bevy 0.8, 0.1 supports Bevy 0.7
* The style options with this release are very limited, only colors and borders.
* Uses the builtin Bevy sprite renderer for drawing.
* 0.2 requires that StarBar/StarBars entities also have the VisibilityBundle components, otherwise they won't be drawn. 

# Usage

In your Cargo.toml `[dependencies]` section, add the line:

```toml
bevy_stat_bars = "0.2"
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
            .spawn_bundle(Camera2dBundle::default())
            .commands()
            .spawn_bundle(StatBarBundle::new(StatBar {
                value: 0.75,
                size: Vec2::new(200., 20.),
                ..Default::default()
            }));
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
* 0.2 update is just a bunch of hacks to get the crate to work with Bevy 0.8. The GlobalTransform stuff especially I don't know what I'm doing there and might be a performance bottleneck if you are drawing 100,000 stat bars every update.

* The math is very hacky and I think there is a problem with the rotations where they don't quite compose in the natural way you'd expect. 

    It's not much of a problem though. Will be fixed next release.


