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

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
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

    pub fn rect(&mut self, r: Rect, color: [f32; 4]) {
        // 4 vertices per rect
        let base = self.vertices.len() as u16;

        let x0 = r.x;
        let y0 = r.y;
        let x1 = r.x + r.w;
        let y1 = r.y + r.h;

        // top-left, top-right, bottom-left, bottom-right
        self.vertices.push(Vertex {
            position: [x0, y0],
            color,
        });
        self.vertices.push(Vertex {
            position: [x1, y0],
            color,
        });
        self.vertices.push(Vertex {
            position: [x0, y1],
            color,
        });
        self.vertices.push(Vertex {
            position: [x1, y1],
            color,
        });

        // two triangles: (0,1,2) (1,3,2)
        self.indices
            .extend_from_slice(&[base, base + 1, base + 2, base + 1, base + 3, base + 2]);
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
