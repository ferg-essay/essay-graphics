struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct StyleInput {
    @location(2) pos: vec2<f32>,
    @location(3) size: vec2<f32>,
    @location(4) r_outer: f32,
    @location(5) color_outer: u32,
    @location(6) r_inner: f32,
    @location(7) color_inner: u32,
}

struct VertexOutput {
    @location(1) xy: vec2<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) size: vec2<f32>,
    @location(4) r_outer: f32,
    @location(5) color_outer: vec4<f32>,
    @location(6) r_inner: f32,
    @location(7) color_inner: vec4<f32>,
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
    out.uv = vertex.uv;

    out.size = style.size;

    out.r_outer = 2. * style.r_outer;
    out.color_outer = unpack_color(style.color_outer);
    out.r_inner = 2. * style.r_inner;
    out.color_inner = unpack_color(style.color_inner);

    return out;
}

@fragment
fn fs_shape(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let sample = textureSample(t_texture, s_texture, in.uv);

    //let dist = max(max(d_x, d_y), dist_corner);
    //let d_xy = in.corner - abs(in.xy);
    //let dist_corner = in.r_outer - length(d_xy);

    //let dist = max(dist_corner, max(d_xy[0], d_xy[1]));
    let d_outer = rect_sdf(in.xy, in.size, in.r_outer, 0.);
    let d_inner = rect_sdf(in.xy, in.size, in.r_outer, in.r_outer - in.r_inner);

    let color_inner = mix(in.color_outer, in.color_inner, clamp(d_inner, 0., 1.));

    return mix(vec4<f32>(0., 0., 0., 0.), color_inner, clamp(d_outer, 0., 1.));
/*    if dist > 0. {
        return vec4<f32>(
            sample.r * in.color[0], 
            sample.g * in.color[1], 
            sample.b * in.color[2], 
            sample.a * in.color[3]
        );
    } else {
        return vec4<f32>(1., 0.2, 0.1, 0.);
    }
    */
}

fn rect_sdf(xy: vec2<f32>, size: vec2<f32>, r_outer: f32, border: f32) -> f32 {
    let corner = size - r_outer;
    let d_xy = corner - abs(xy);
    let dist_corner = r_outer - border - length(d_xy);

    return max(dist_corner, max(
        min(d_xy[0], size[1] - abs(xy[1]) - border),
        min(d_xy[1], size[0] - abs(xy[0]) - border)
    ));
    //    min(d_xy[1], border - abs(xy[0]))
    //return max(
    //    min(d_xy[0], border - abs(xy[1])),
    //    min(d_xy[1], border - abs(xy[0]))
    //);
    //return max(dist_corner, max(d_xy[0], d_xy[1]));
    //return dist_corner;
}

fn unpack_color(color: u32) -> vec4<f32> {
    return vec4<f32>(
        f32((color >> 24u) & 0xffu),
        f32((color >> 16u) & 0xffu),
        f32((color >> 8u) & 0xffu),
        f32(color & 0xffu),
    ) / 255.0;
}
