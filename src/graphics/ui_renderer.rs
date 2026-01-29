use crate::ui::geometry;
use glam::Mat4;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [u8; 4],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
            wgpu::vertex_attr_array![0 => Float32x2, 1 => Unorm8x4, 2 => Float32x2];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

use image::GenericImageView;

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Self {
        let img = image::load_from_memory(bytes);
        Self::from_image(device, queue, &img.unwrap(), Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
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

    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,

    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group: wgpu::BindGroup,

    index_count: u32,
}

impl UiRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let diffuse_bytes = include_bytes!("happy-tree.png");
        let test_texture =
            Texture::from_bytes(device, queue, diffuse_bytes, "uh buh bu - STWB (who else)");

        let size = 64;
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("STRWB - uniform buffer"),
            size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform bind group layout - STRWB"),
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

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("STRWB - uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture bind group layout - STRWB"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&test_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&test_texture.sampler),
                },
            ],
            label: Some("STRWB - texture_bind_group"),
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

        Self {
            vertices: Vec::with_capacity(vertex_buffer_capacity),
            indices: Vec::with_capacity(index_buffer_capacity),
            vertex_buffer,
            index_buffer,
            vertex_buffer_capacity,
            index_buffer_capacity,
            uniform_buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            texture_bind_group,
            index_count: 0,
        }
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
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);

        pass.set_bind_group(1, &self.texture_bind_group, &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }

    pub fn rect(&mut self, rect: &geometry::Rect) {
        let vertex_offset = self.vertices.len() as u16;

        let left = rect.x;
        let top = rect.y;
        let right = rect.x + rect.w;
        let bottom = rect.y + rect.h;

        self.vertices.push(Vertex {
            position: [left, top],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [1.0, 1.0],
        });

        self.vertices.push(Vertex {
            position: [right, top],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [1.0, 1.0],
        });

        self.vertices.push(Vertex {
            position: [left, bottom],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [1.0, 1.0],
        });

        self.vertices.push(Vertex {
            position: [right, bottom],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [1.0, 1.0],
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

    pub fn begin_frame(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.vertices.clear();
        self.indices.clear();
        self.index_count = 0;

        let projection = Mat4::orthographic_rh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0)
            .to_cols_array_2d();
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&projection));
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

        self.vertices.push(Vertex {
            position: [50.0, 25.0],
            color: [255;4],
            uv: [0.4131759, 0.99240386],
        });
        self.vertices.push(Vertex {
            position: [25.0, 50.0],
            color: [255;4],
            uv: [0.0048659444, 0.1234],
        });
        self.vertices.push(Vertex {
            position: [40.0, 75.0],
            color: [255;4],
            uv: [0.28081453, 0.05060294],
        });
        self.vertices.push(Vertex {
            position: [60.0, 60.0],
            color: [255;4],
            uv: [0.85967, 0.1526709],
        });
        self.vertices.push(Vertex {
            position: [75.0, 40.0],
            color: [255;4],
            uv: [0.9414737, 0.7347359],
        });

        let vertex_offset = self.vertices.len() as u16;

        self.indices.extend_from_slice(&[
            vertex_offset, vertex_offset+1, vertex_offset+4,
            vertex_offset+1, vertex_offset+2, vertex_offset+4,
            vertex_offset+2, vertex_offset+3, vertex_offset+4,
            0,
        ]);

        self.ensure_capacity(device);
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        self.index_count = self.indices.len() as u32;
    }
}
