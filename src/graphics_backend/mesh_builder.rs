use glm::*;
use wgpu::util::DeviceExt;

#[repr(C)]
pub struct Vertex {
    position: Vec3,
    color: Vec3,
}

pub struct Mesh{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {

        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

unsafe fn convert_to_byte_array<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }
}

pub fn make_quad(device: &wgpu::Device) -> Mesh {

    let bottom_left= Vec2::new(-0.5, -0.5);
    let bottom_right= Vec2::new(0.5, -0.5);
    let top_left = Vec2::new(-0.5, 0.5);
    let top_right= Vec2::new(0.5, 0.5);

    let vertacies: [Vertex; 4] = [
        Vertex {position: Vec3::new(bottom_left.x, bottom_left.y, 0.0), color: Vec3::new(1.0, 0.0, 1.0)},
        Vertex {position: Vec3::new(bottom_right.x, bottom_right.y, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
        Vertex {position: Vec3::new(top_left.x, top_left.y, 0.0), color: Vec3::new(0.0, 1.0, 0.0)},
        Vertex {position: Vec3::new(top_right.x, top_right.y, 0.0), color: Vec3::new(0.0, 0.0, 1.0)},
    ];
    let vertacies_as_bytes: &[u8] = unsafe {
        convert_to_byte_array(& vertacies)
    };

    let vertex_buffer_descriptor = wgpu::util::BufferInitDescriptor{
        label: Some("quad vertex buffer - STRWB"),
        contents: &vertacies_as_bytes,
        usage: wgpu::BufferUsages::VERTEX
    };

    let indicies: [u16; 6] = [0, 1, 2, 1, 2, 3];
    let indacies_as_btes: &[u8] = unsafe{
        convert_to_byte_array(& indicies)
    };

    let index_buffer_descriptor = wgpu::util::BufferInitDescriptor{
        label: Some("index buffer - STRWB"),
        contents: &indacies_as_btes,
        usage: wgpu::BufferUsages::INDEX
    };

    let index_buffer = device.create_buffer_init(&index_buffer_descriptor);
    let vertex_buffer = device.create_buffer_init(&vertex_buffer_descriptor);
    
    Mesh {vertex_buffer: vertex_buffer, index_buffer: index_buffer}
}