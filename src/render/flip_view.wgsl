#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@fragment
fn x(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    return textureSample(screen_texture, texture_sampler, in.uv * vec2<f32>(-1.0, 1.0));
}

@fragment
fn y(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    return textureSample(screen_texture, texture_sampler, in.uv * vec2<f32>(1.0, -1.0));
}
