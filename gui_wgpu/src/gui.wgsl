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
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0);
}
