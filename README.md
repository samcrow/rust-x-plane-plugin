
# X-Plane plugin macro for Rust

## Purpose

This library provides a macro for easy creation of plugins for X-Plane.

With this library and the [xplm](https://crates.io/crates/xplm)
crate, X-Plane plugins can be easily developed in Rust.

## Use

To import the macro, the crate must be imported like this:

```
#[macro_use]
extern crate xplane_plugin;
```

Creating a plugin involves three steps:

1. Create a struct for your plugin
2. Implement `Plugin` for your plugin struct
3. Place `xplane_plugin!(YourPluginStruct)` in a file, not in any function

## Examples

```
#[macro_use]
extern crate xplane_plugin;
use xplane_plugin::*;
struct TestPlugin;
impl Plugin for TestPlugin {
    fn start() -> Option<Self> {
        Some(TestPlugin)
    }
    fn enable(&mut self) {

    }
    fn disable(&mut self) {

    }

    fn stop(&mut self) {

    }
    fn info<'a, 'b, 'c>(&self) -> PluginInfo<'a, 'b, 'c> {
        PluginInfo {
            name: "Test Plugin",
            signature: "org.samcrow.rustplugin.test",
            description: "A plugin written in Rust",
        }
    }
}

xplane_plugin!(TestPlugin);
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
