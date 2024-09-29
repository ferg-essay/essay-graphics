//
// 3d triangles for 3d
//

struct Camera {
    a0: vec3<f32>,
    a1: vec3<f32>,
}

@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) tex_uv: vec2<f32>,
}

struct VertexOutput {
    @location(0) tex_uv: vec2<f32>,
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_shape2d(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let a0 = camera.a0;
    let a1 = camera.a1;
    let xp = model.pos[0];
    let yp = model.pos[1];
    let x = a0[0] * xp + a0[1] * yp + a0[2];
    let y = a1[0] * xp + a1[1] * yp + a1[2];
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_uv = model.tex_uv;
    return out;
}

@group(0) @binding(0)
var t_texture: texture_2d<f32>;

@group(0) @binding(1)
var s_texture: sampler;

@fragment
fn fs_shape2d(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.tex_uv);
}
