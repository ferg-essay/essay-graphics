//
// 3d triangles for 3d
//

struct Camera {
    view_proj: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_uv: vec2<f32>,
}

struct StyleInput {
    @location(2) dummy: u32,
}

struct VertexOutput {
    @location(0) tex_uv: vec2<f32>,
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_form3d(
    model: VertexInput,
    style: StyleInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = camera.view_proj * vec4<f32>(model.pos, 1.0);
    out.tex_uv = model.tex_uv;
    return out;
}

@group(0) @binding(0)
var t_texture: texture_2d<f32>;

@group(0) @binding(1)
var s_texture: sampler;

@fragment
fn fs_form3d(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.tex_uv);
}
