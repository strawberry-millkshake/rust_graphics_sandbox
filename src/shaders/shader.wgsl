struct Uniforms {
    projection : mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> globals: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertexInput: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = globals.projection * vec4<f32>(vertexInput.position, 0.0, 1.0);
    output.color = vertexInput.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}