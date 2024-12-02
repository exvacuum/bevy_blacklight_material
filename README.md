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

## Installation

### crates.io
```toml
[dependencies]
bevy_blacklight_material = "0.1"
```

### Using git URL in Cargo.toml
```toml
[dependencies.bevy_rustysynth]
git = "https://github.com/exvacuum/bevy_blacklight_material.git"
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
commands.spawn(MaterialMeshBundle {
    material: asset_server.add(BlacklightMaterial {
        // base texture, color, etc
        ..Default::default()
    }),
    ..Default::default()
});


// Blacklight
commands.spawn((
    Blacklight, // Marker component
    SpotLightBundle {
        spot_light: SpotLight {
            // outer/inner angle, range
            ..Default::default()
        },
        ..Default::default()     
    },
));
```

## License

This crate is licensed under your choice of 0BSD, Apache-2.0, or MIT license.

