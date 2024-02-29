// @group(0) @binding(0) var input_texture : texture_2d<f32>;
// @group(0) @binding(1) var output_texture : texture_storage_2d<rgba8unorm, write>;

// @compute
// @workgroup_size(16, 16)
// fn main(
//     @builtin(global_invocation_id) global_id: vec3<u32>,
// ) {
//     let dimensions = textureDimensions(input_texture);
//     if i32(global_id.x) >= dimensions.x || i32(global_id.y) >= dimensions.y {
//         return;
//     }

//     let target_position = vec2<i32>(i32(global_id.x), dimensions.y - i32(global_id.y) - 1);
//     let color = textureLoad(input_texture, vec2<i32>(global_id.xy), 0);

//     textureStore(output_texture, vec2<i32>(target_position), color);
// }




// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    return textureSample(screen_texture, texture_sampler, in.uv * vec2<f32>(1.0, -1.0));
}
