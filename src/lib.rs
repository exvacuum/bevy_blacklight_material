#![warn(missing_docs)]

//! A plugin for the bevy engine providing a simple [`BlacklightMaterial`], which reveals a base
//! color based on light data from spot lights tagged with a [`Blacklight`] component.
//! 
//! Possible future features:
//! - [`StandardMaterial`] extension for a "blacklight mapped" material.
//! - Point light support.

use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};

/// Plugin which enables and updates blacklight shaders.
pub struct BlacklightPlugin;

impl Plugin for BlacklightPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/shaders/blacklight_material.wgsl");
        app.add_plugins(MaterialPlugin::<BlacklightMaterial>::default())
            .add_systems(Update, update_shader_blacklight_data);
    }
}

/// Marker component for spot lights to use them as blacklights for [`BlacklightMaterial`].
#[derive(Component, Debug)]
pub struct Blacklight;

/// Shader data representing a single blacklight point light.
#[derive(Clone, Debug, ShaderType)]
pub struct BlacklightData {
    /// World-space position of this light.
    pub position: Vec3,
    /// World-space direction of this light (must be normalized).
    pub direction: Vec3,
    /// Range of this light (see [`SpotLight`]).
    pub range: f32,
    /// Inner angle of this light (see [`SpotLight`]).
    pub inner_angle: f32,
    /// Outer angle of this light (see [`SpotLight`]).
    pub outer_angle: f32,
}

/// Material which is invisible until exposed to light from a spot light tagged with the
/// [`Blacklight`] component.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlacklightMaterial {
    /// List of light data processed by this shader
    #[storage(0, read_only)]
    pub lights: Vec<BlacklightData>,
    /// Base color texture which is revealed by blacklight exposure.
    #[texture(1)]
    #[sampler(2)]
    pub base_texture: Option<Handle<Image>>,
    /// Base color of material, multiplies with texture.
    #[uniform(3)]
    pub base_color: LinearRgba,
    /// Alpha mode for this material.
    pub alpha_mode: AlphaMode,
}

impl Default for BlacklightMaterial {
    fn default() -> Self {
        Self {
            lights: vec![],
            base_texture: None,
            base_color: LinearRgba::WHITE,
            alpha_mode: AlphaMode::Blend,
        }
    }
}

impl Material for BlacklightMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "embedded://bevy_blacklight_material/../assets/shaders/blacklight_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn update_shader_blacklight_data(
    blacklight_query: Query<(&ViewVisibility, &GlobalTransform, &SpotLight), With<Blacklight>>,
    blacklight_material_query: Query<&Handle<BlacklightMaterial>>,
    mut blacklight_materials: ResMut<Assets<BlacklightMaterial>>,
) {
    let light_data = blacklight_query
        .iter()
        .filter(|(visibility, _, _)| visibility.get())
        .map(|(_, global_transform, light)| BlacklightData {
            position: global_transform.translation(),
            direction: *global_transform.forward(),
            range: light.range,
            inner_angle: light.inner_angle,
            outer_angle: light.outer_angle,
        })
        .collect::<Vec<_>>();
    for handle in blacklight_material_query.iter() {
        let material = blacklight_materials.get_mut(handle).unwrap();
        material.lights = light_data.clone();
    }
}
