use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub projection: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
}
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
pub struct Mesh{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {

        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

pub fn make_quad(device: &wgpu::Device) -> Mesh {

    let vertacies: [Vertex; 4] = [
        Vertex {position: [0.0, 0.0], color: [1.0, 0.0, 1.0] },
        Vertex {position: [1.0, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex {position: [0.0, 1.0], color: [0.0, 1.0, 0.0] },
        Vertex {position: [1.0, 1.0], color: [0.0, 0.0, 1.0] },
    ];

    let vertex_buffer_descriptor = wgpu::util::BufferInitDescriptor{
        label: Some("quad vertex buffer - STRWB"),
        contents: bytemuck::cast_slice(&vertacies),
        usage: wgpu::BufferUsages::VERTEX
    };

    let indicies: [u16; 6] = [0, 1, 2, 1, 2, 3];

    let index_buffer_descriptor = wgpu::util::BufferInitDescriptor{
        label: Some("index buffer - STRWB"),
        contents: bytemuck::cast_slice(&indicies),
        usage: wgpu::BufferUsages::INDEX
    };

    let index_buffer = device.create_buffer_init(&index_buffer_descriptor);
    let vertex_buffer = device.create_buffer_init(&vertex_buffer_descriptor);
    
    Mesh {vertex_buffer: vertex_buffer, index_buffer: index_buffer, index_count: indicies.len() as u32}
}