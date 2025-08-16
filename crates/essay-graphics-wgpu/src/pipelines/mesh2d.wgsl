struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct StyleInput {
    @location(2) a0: vec4<f32>,
    @location(3) a1: vec4<f32>,
    @location(4) color: vec4<f32>,
}

struct VertexOutput {
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @builtin(position) pos: vec4<f32>,
};

@group(0) @binding(0)
var t_texture: texture_2d<f32>;

@group(0) @binding(1)
var s_texture: sampler;

@vertex
fn vs_shape(
    model: VertexInput,
    style: StyleInput,
) -> VertexOutput {
    let a0 = style.a0;
    let a1 = style.a1;
    let x0 = model.pos[0];
    let y0 = model.pos[1];
    let x = a0[0] * x0 + a0[1] * y0 + a0[3];
    let y = a1[0] * x0 + a1[1] * y0 + a1[3];

    var out: VertexOutput;
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    out.color = style.color;
    out.uv = model.uv;

    return out;
}

@fragment
fn fs_shape(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let sample = textureSample(t_texture, s_texture, in.uv);

    return vec4<f32>(
        sample.r * in.color[0], 
        sample.g * in.color[1], 
        sample.b * in.color[2], 
        sample.a * in.color[3]
    );
}
