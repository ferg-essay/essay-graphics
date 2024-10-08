struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
}

struct StyleInput {
    @location(2) a0: vec4<f32>,
    @location(3) a1: vec4<f32>,
    @location(4) color: vec4<f32>,
}

struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_shape_tex(
    model: VertexInput,
    style: StyleInput,
) -> VertexOutput {
    let a0 = style.a0;
    let a1 = style.a1;
    let xp = model.pos[0];
    let yp = model.pos[1];
    let x = a0[0] * xp + a0[1] * yp + a0[3];
    let y = a1[0] * xp + a1[1] * yp + a1[3];
    var out: VertexOutput;
    out.tex_coord = model.tex_coord;
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    out.color = style.color;
    return out;
}

@group(0) @binding(0)
var t_texture: texture_2d<f32>;

@group(0) @binding(1)
var s_texture: sampler;

@fragment
fn fs_shape_tex(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let alpha = textureSample(t_texture, s_texture, in.tex_coord).r;

    return vec4<f32>(in.color[0], in.color[1], in.color[2], alpha * in.color[3]);
}
