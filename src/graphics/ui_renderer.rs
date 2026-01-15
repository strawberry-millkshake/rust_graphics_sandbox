use crate::ui::geometry;
use glam::Mat4;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    projection: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

pub struct UiRenderer {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    vertex_buffer_capacity: usize,
    index_buffer_capacity: usize,

    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,

    index_count: u32,
}

impl UiRenderer {
    pub fn new(device: &wgpu::Device) -> (Self, wgpu::BindGroupLayout) {
        let globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ui globals bgl - STRWB"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let globals = Globals {
            projection: Mat4::IDENTITY.to_cols_array_2d(),
        };

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui globals buffer - STRWB"),
            contents: bytemuck::bytes_of(&globals),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui globals bg - STRWB"),
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buffer.as_entire_binding(),
            }],
        });

        let vertex_buffer_capacity = 1024;
        let index_buffer_capacity = 2048;

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ui vertex buffer - STRWB"),
            size: (vertex_buffer_capacity * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ui index buffer - STRWB"),
            size: (index_buffer_capacity * std::mem::size_of::<u16>()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        (
            Self {
                vertices: Vec::with_capacity(vertex_buffer_capacity),
                indices: Vec::with_capacity(index_buffer_capacity),
                vertex_buffer,
                index_buffer,
                vertex_buffer_capacity,
                index_buffer_capacity,
                globals_buffer,
                globals_bind_group,
                index_count: 0,
            },
            globals_bind_group_layout,
        )
    }

    pub fn begin_frame(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.vertices.clear();
        self.indices.clear();
        self.index_count = 0;

        let projection = Mat4::orthographic_rh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        let globals = Globals {
            projection: projection.to_cols_array_2d(),
        };

        queue.write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&globals));
    }

    pub fn rect(&mut self, rect: &geometry::Rect) {
        let vertex_offset = self.vertices.len() as u16;

        let left = rect.x;
        let top = rect.y;
        let right = rect.x + rect.w;
        let bottom = rect.y + rect.h;

        self.vertices.push(Vertex {
            position: [left, top],
            color: [
                (rect.color.r / 255) as f32,
                (rect.color.g / 255) as f32,
                (rect.color.b / 255) as f32,
                (rect.color.a / 255) as f32,
            ],
        });

        self.vertices.push(Vertex {
            position: [right, top],
            color: [
                (rect.color.r / 255) as f32,
                (rect.color.g / 255) as f32,
                (rect.color.b / 255) as f32,
                (rect.color.a / 255) as f32,
            ],
        });

        self.vertices.push(Vertex {
            position: [left, bottom],
            color: [
                (rect.color.r / 255) as f32,
                (rect.color.g / 255) as f32,
                (rect.color.b / 255) as f32,
                (rect.color.a / 255) as f32,
            ],
        });

        self.vertices.push(Vertex {
            position: [right, bottom],
            color: [
                (rect.color.r / 255) as f32,
                (rect.color.g / 255) as f32,
                (rect.color.b / 255) as f32,
                (rect.color.a / 255) as f32,
            ],
        });

        // two triangles: (0,1,2) (1,3,2)
        self.indices.extend_from_slice(&[
            vertex_offset,
            vertex_offset + 1,
            vertex_offset + 2,
            vertex_offset + 1,
            vertex_offset + 3,
            vertex_offset + 2,
        ]);
    }

    fn ensure_capacity(&mut self, device: &wgpu::Device) {
        if self.vertices.len() > self.vertex_buffer_capacity {
            while self.vertices.len() > self.vertex_buffer_capacity {
                self.vertex_buffer_capacity *= 2;
            }
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ui vertex buffer (grown) - STRWB"),
                size: (self.vertex_buffer_capacity * std::mem::size_of::<Vertex>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        if self.indices.len() > self.index_buffer_capacity {
            while self.indices.len() > self.index_buffer_capacity {
                self.index_buffer_capacity *= 2;
            }
            self.index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ui index buffer (grown) - STRWB"),
                size: (self.index_buffer_capacity * std::mem::size_of::<u16>()) as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
    }

    pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.ensure_capacity(device);

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        self.index_count = self.indices.len() as u32;
    }

    pub fn draw<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        pipeline: &'pass wgpu::RenderPipeline,
    ) {
        if self.index_count == 0 {
            return;
        }
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &self.globals_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
