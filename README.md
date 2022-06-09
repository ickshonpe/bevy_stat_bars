# Bevy Stat Bars

Simple library for drawing floating stat bars.

## 
![](3.png)

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
This a minimal system that should spawn a horizontal 75% filled stat bar in the middle of the screen (assuming you already have an app with a 2d camera):

```rust
use bevy_stat_bars::*;

fn spawn_a_stat_bar(mut commands: Commands) {
    commands.spawn_bundle((
        StatBar {
            value: 0.75,
            size: Vec2::new(200., 20.),
            ..Default::default()
        },
    ));
}
```

There are some complete examples you can run with the commands:
```
cargo run --example basic
cargo run --example interactive
cargo run --example minimal
```


# Notes
* The math is very hacky and I think there is a bug I think with the rotations where they don't quite compose in the natural way you'd expect. 

    It's not much of a problem though. Will be fixed next release.


