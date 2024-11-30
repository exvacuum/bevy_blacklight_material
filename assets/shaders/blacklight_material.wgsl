#import bevy_pbr::forward_io::VertexOutput;

struct BlackLight {
	position: vec3<f32>,
	direction: vec3<f32>,
	range: f32,
	inner_angle: f32,
	outer_angle: f32,
}

@group(2) @binding(0) var<storage> lights: array<BlackLight>;
@group(2) @binding(1) var base_texture: texture_2d<f32>;
@group(2) @binding(2) var base_sampler: sampler;

@fragment
fn fragment(
	in: VertexOutput,
) -> @location(0) vec4<f32> {
	let base_color = textureSample(base_texture, base_sampler, in.uv);
	var final_color = vec4f(0.0, 0.0, 0.0, 0.0);
	for (var i = u32(0); i < arrayLength(&lights); i = i+1) {
		let light = lights[i];

		let light_to_fragment_direction = normalize(in.world_position.xyz - light.position);
		let light_to_fragment_angle = acos(dot(light.direction, light_to_fragment_direction));
		let angle_inner_factor = light.inner_angle / light.outer_angle;
		let angle_factor = linear_falloff_radius(light_to_fragment_angle / light.outer_angle, angle_inner_factor);

		let light_distance_squared = distance_squared(in.world_position.xyz, light.position);
		let distance_factor = inverse_falloff_radius(saturate(light_distance_squared / (light.range * light.range)), 0.5);

		final_color = saturate(final_color + base_color * angle_factor * distance_factor);
	}
	return final_color;
}

fn distance_squared(a: vec3f, b: vec3f) -> f32 {
	let vec = a - b;
	return dot(vec, vec);
}

fn linear_falloff_radius(factor: f32, radius: f32) -> f32 {
	if factor < radius {
		return 1.0;
	} else {
		return 1.0 - (factor - radius) / (1.0 - radius);
	}
}

fn inverse_falloff(factor: f32) -> f32 {
	return pow(1.0 - factor, 2.0);
}

fn inverse_falloff_radius(factor: f32, radius: f32) -> f32 {
	if factor < radius {
		return 1.0;
	} else {
		return inverse_falloff((factor - radius) / (1.0 - radius));
	}
}
