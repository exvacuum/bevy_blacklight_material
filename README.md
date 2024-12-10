# bevy_blacklight_material

[![Crates](https://img.shields.io/crates/v/bevy_blacklight_material)](https://crates.io/crates/bevy_blacklight_material)
![License](https://img.shields.io/badge/license-0BSD%2FMIT%2FApache-blue.svg)
![Tag](https://img.shields.io/github/v/tag/exvacuum/bevy_blacklight_material)
![Build](https://img.shields.io/github/actions/workflow/status/exvacuum/bevy_blacklight_material/rust.yml)

A plugin for the [Bevy Engine](https://bevyengine.org) which adds a "blacklight" material that is revealed by spot lights marked with a `Blacklight` component.

Feel free to contribute if you want to improve this, it was thrown together pretty hastily so there's bound to be some errors or oversights.

## Compatibility

| Crate Version | Bevy Version |
|---            |---           |
| 0.1           | 0.14         |
| 0.2           | 0.15         |

## Installation

### crates.io
```toml
[dependencies]
bevy_blacklight_material = "0.2"
```

### Using git URL in Cargo.toml
```toml
[dependencies.bevy_rustysynth]
git = "https://git.exvacuum.dev/bevy_blacklight_material"
```

## Usage

In `main.rs`:
```rs
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlacklightPlugin,
        ))
        .run();
}
```
Then you can create blacklight-emitting spotlights, and reveal-able surfaces, like this:
```rs
// Mesh with blacklight material
commands.spawn((
    //...
    MeshMaterial3d(asset_server.add(BlacklightMaterial::new(&asset_server, None, Color::WHITE))),
));


// Blacklight
// Requires `SpotLight`, but you might want to add one yourself
commands.spawn(Blacklight);
```

## License

This crate is licensed under your choice of 0BSD, Apache-2.0, or MIT license.

