# bevy_stat_bars

Bevy crate for drawing floating statbars like health bars above enemy sprites etc.

## Version 0.3

* Supports Bevy 0.8

### New in this release

* Completely rewritten. New design and API. 
* Should (hopefully) be easier to use, the redesign seems better to me but let me know if you hate the changes. 
* (Seems to) Work nicely with ```bevy_inspector_egui``` now.
* Removed the arbitrary orientation stuff temporarily, just has reversible horizontal and
 vertical bars.
* Statbars can track resources as well as components.
* No plugin, need to add an observer to your Bevy ```App`` for each type of Statbar before they will draw.
* Multiple Statbar components on one entity implemented using PhantomData. This requires
 ![/media/example.png](/media/example.png)

# 

## How to use

Add the dependency to your Cargo.toml file with

```toml
[dependencies.bevy_stat_bars]
version = "0.3"
```

Then register any components you want to observe with a statbar with your Bevy App:

```rust 
use bevy_stat_bars::*;

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

There are six examples you can look at that cover most of the features and use cases,
run them with
```
cargo run --example minimal_standalone
cargo run --example basic_interactive
cargo run --example observe_resource
cargo run --example demo
cargo run --example stress --release
cargo run --example stress2 --release
```
The ```demo``` example is the probably the most useful to look at.

The ```stress2``` example uses macros to add hundreds of marker types and can take a few minutes to compile.

#

## Notes

* Only supports 2D.

* When I was writing the examples I made a mistake where instead of

    ```rust
    .add_statbar_component_observer::<Stat<Health>>()
    ```
    I used 
    ```rust
    .add_statbar_component_observer::<Health>()
    ```
    which is quite easy to miss. The crate fails silently and just won't render anything in this case, leaving the user with a frustrating bug hunt. 

    Likewise also when a statbar is set to observe its parent or another Entity that doesn't exist, it will render a statbar that doesn't update. 

* Statbars are drawn using Sprites with a z depth of 990, and if you translate the camera down more than 10 units they won't draw.
You can change the depth with the ```StatbarDepth``` resource. 
    
    So with

    ```rust
    commands.insert_resource(StatbarDepth(500.));
    ```

    all Statbars will now render with a z depth of 500.
    There currently isn't any way to control the ordering in which the individual statbars are drawn.

* Still uses sprites for rendering which isn't ideal but performance seems fine. You can run the ```stress``` example to see what its like under a heavy load. I get about 100fps on my rx580.

* ```add_statbar_component_observer``` adds six systems to your Bevy app per component observed. Again not ideal but doesn't seem to be a problem. I get ~100fps with the ```stress2``` example which spawns 100 entities with 200 Statbars each.
#
## Future Plans

* Replace the sprite based rendering with a custom renderer. I have some fragment shaders already written, and should be better performance with some nice effects like rounded corners and color gradients.
* Pie-o-meters
* Labels and numeric indicators
* Some sort of, posibly feature gated or debug-only, falure detection that gives an error when you insert unregistered statbars, or when a statbar can't find the component it is meant to be observing.
* Derive macro for StatbarObservable.
* Auto arrangement/stacking of groups of statbars. I thought this would be more difficult but I dreamt up an easyish way to do it last night.











