# bevy_stat_bars

## Version 0.3

New stuff

* Completely rewritten. New design and API. Should (hopefully) be easier to use.
* Works nicely with ```bevy_inspector_egui``` now.
* Removed the arbitrary orientation stuff temporarily, just has reversible horizontal and
 vertical bars.
* Statbars can track resources as well as components.
* No plugin, need to register types tracked by Statbars before you can use them though.

 ![/media/example.png](/media/example.png)

# 

## How to use

Add the dependency to your Cargo.toml file with

```toml
[dependencies.bevy_stat_bars]
version = "0.3"
```

Then register any components you want to observe with a Statbar with your Bevy App:

```rust 
App::new()
    .add_plugins(DefaultPlugins)
    .add_statbar_bar_component_observer::<HitPoints>()
    // ..etc, rest of app
    .run();
```

You also need to implement the ```StatbarObservable``` trait on those components:

```rust 
impl StatbarObservable for HitPoints {
    fn get_statbar_value(&self) -> f32 {
        self.value / self.max
    }
}
```

And now you can add a ```Statbar::<HitPoints>``` component to an entity to visualize its HitPoints component

```rust
commands.entity(enemy_id)
    .insert_bundle((
        Statbar::<HitPoints> {
            empty_color: Color::NAVY,
            length: 10.,
            thickness: 2.,
            displacement: 8. * Vec2::Y,
            ..Default::default()
        },
        StatbarBorder::<HitPoints>::all(Color::WHITE, 1.),
    ));
```

![/media/example2.png](/media/example2.png)

#

## Examples

There are five examples you can look at that cover most of the features and use cases,
run them with
```
cargo run --example minimal_standalone
cargo run --example basic_interactive
cargo run --example observe_resource
cargo run --example demo
cargo run --example stress --release
```
The ```demo``` example is the probably the most useful to look at.

#

## Notes

* Supports Bevy 0.8
* Still uses sprites for rendering which isn't ideal but performance seems fine. You can run the ```stress``` example to see what its like under a heavy load. I get about 100fps on my rx580.












