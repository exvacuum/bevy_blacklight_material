#import bevy_pbr::forward_io::VertexOutput;

struct BlackLight {
	position: vec3<f32>,
	direction: vec3<f32>,
	color: vec4<f32>,
	range: f32,
	radius: f32,
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
		let light_distance_squared = distance_squared(in.world_position.xyz, light.position);
		let light_arccosine = abs(acos(dot(normalize(light.direction), normalize(in.world_position.xyz - light.position)))) * radians(180.0);
		final_color = saturate(final_color + base_color * (inverse_falloff_radius(light_distance_squared / (light.range * light.range), 0.5) * inverse_falloff_radius(light_arccosine, 0.9)));
	}
	return final_color;
}

fn distance_squared(a: vec3f, b: vec3f) -> f32 {
	return pow(a.x - b.x, 2.0) + pow(a.y - b.y, 2.0) + pow(a.z - b.z, 2.0);
}

fn inverse_falloff(factor: f32) -> f32 {
	let squared = factor * factor;
	return (1.0 - squared) / (10 * squared + 1.0);
}

fn inverse_falloff_radius(factor: f32, radius: f32) -> f32 {
	if factor < radius {
		return 1.0;
	} else {
		return inverse_falloff((factor - radius) / (1.0 - radius));
	}
}
