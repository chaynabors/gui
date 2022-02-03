struct Globals {
    resolution: vec2<u32>;
    scale_factor: f32;
};

struct Locals {
    tex_coord: vec2<i32>;
    color: u32;
};

[[group(0), binding(0)]]
var<uniform> globals: Globals;
[[group(0), binding(1)]]
var<uniform> locals: Locals;
[[group(0), binding(2)]]
var albedo: texture_2d<f32>;

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(
        f32(i32(vertex_index) & 1) * 4.0 - 1.0,
        f32(i32(vertex_index) / 2) * 4.0 - 1.0,
        1.0,
        1.0
    );
}

[[stage(fragment)]]
fn fs_main([[builtin(position)]] position: vec4<f32>) -> [[location(0)]] vec4<f32> {
    let albedo_dimensions = textureDimensions(albedo);
    let color = textureLoad(albedo, vec2<i32>((position.xy - vec2<f32>(locals.tex_coord)) / globals.scale_factor) % albedo_dimensions, 0) * unpack4x8unorm(locals.color);
    return color;
}
