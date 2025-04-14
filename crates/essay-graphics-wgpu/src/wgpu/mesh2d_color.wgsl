struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) color: u32,
}

struct StyleInput {
    @location(2) a0: vec4<f32>,
    @location(3) a1: vec4<f32>,
}

struct VertexOutput {
    @location(1) color: vec4<f32>,
    @builtin(position) pos: vec4<f32>,
};

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
    out.color = unpack_color(model.color);

    return out;
}

@fragment
fn fs_shape(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return in.color;
}

fn unpack_color(color: u32) -> vec4<f32> {
    return vec4<f32>(
        f32((color >> 24u) & 0xffu),
        f32((color >> 16u) & 0xffu),
        f32((color >> 8u) & 0xffu),
        f32(color & 0xffu),
    ) / 255.0;
}
