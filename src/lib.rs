use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};

pub struct BlacklightPlugin;

impl Plugin for BlacklightPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/shaders/blacklight_material.wgsl");
        app.add_plugins(MaterialPlugin::<BlacklightMaterial>::default()).add_systems(Update, update_shader_blacklight_data);
    }
}

#[derive(Component, Debug)]
pub struct Blacklight;

#[derive(Clone, Debug, ShaderType)]
pub struct BlacklightData {
    pub position: Vec3,
    pub direction: Vec3,
    pub range: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlacklightMaterial {
    #[storage(0, read_only)]
    pub lights: Vec<BlacklightData>,
    #[texture(1)]
    #[sampler(2)]
    pub base_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Default for BlacklightMaterial {
    fn default() -> Self {
        Self {
            lights: vec![],
            base_texture: None,
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
