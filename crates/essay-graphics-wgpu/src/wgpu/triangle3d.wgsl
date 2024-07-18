//
// 3d triangles for 3d
//

struct Camera {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) pos: vec3<f32>,
}

struct StyleInput {
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @location(0) color: vec4<f32>,
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_triangle(
    model: VertexInput,
    style: StyleInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = camera.view_proj * vec4<f32>(model.pos, 1.0);
    //out.pos = vec4<f32>(model.pos, 1.0) * camera.view_proj;
    out.color = style.color;
    return out;
}

@fragment
fn fs_triangle(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return in.color;
}
