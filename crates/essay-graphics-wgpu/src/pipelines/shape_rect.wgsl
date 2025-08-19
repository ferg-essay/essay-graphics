struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct StyleInput {
    @location(2) pos: vec2<f32>,
    @location(3) size: vec2<f32>,
    @location(4) r0: f32,
    @location(5) color: u32,
}

struct VertexOutput {
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) xy: vec2<f32>,
    @location(4) corner: vec2<f32>,
    @location(5) r: f32,
    @builtin(position) pos: vec4<f32>,
};

@group(0) @binding(0)
var t_texture: texture_2d<f32>;

@group(0) @binding(1)
var s_texture: sampler;

struct Viewport {
    size: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> viewport: Viewport;

@vertex
fn vs_shape(
    vertex: VertexInput,
    style: StyleInput,
) -> VertexOutput {
    let pos = (vertex.pos * style.size + style.size) + 2. * style.pos;

    let out_x = (pos[0] - viewport.size[0]) / viewport.size[0];
    let out_y = - (pos[1] - viewport.size[1]) / viewport.size[1];

    var out: VertexOutput;
    out.pos = vec4<f32>(out_x, out_y, 0.0, 1.0);
    out.xy = vertex.pos * style.size;
    out.corner = max(style.size - 2. * style.r0, vec2<f32>(0., 0.));
    out.r = 2. * style.r0;
    out.uv = vertex.uv;
    out.color = unpack_color(style.color);

    return out;
}

@fragment
fn fs_shape(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let sample = textureSample(t_texture, s_texture, in.uv);

    //let dist = max(max(d_x, d_y), dist_corner);
    let d_xy = in.corner - abs(in.xy);
    let dist_corner = in.r - length(d_xy);

    let dist = max(dist_corner, max(d_xy[0], d_xy[1]));

    if dist > 0. {
        return vec4<f32>(
            sample.r * in.color[0], 
            sample.g * in.color[1], 
            sample.b * in.color[2], 
            sample.a * in.color[3]
        );
    } else {
        return vec4<f32>(1., 0.2, 0.1, 0.);
    }
}

fn unpack_color(color: u32) -> vec4<f32> {
    return vec4<f32>(
        f32((color >> 24u) & 0xffu),
        f32((color >> 16u) & 0xffu),
        f32((color >> 8u) & 0xffu),
        f32(color & 0xffu),
    ) / 255.0;
}
