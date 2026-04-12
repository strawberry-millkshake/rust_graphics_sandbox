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

    pub fn from_null(device: &wgpu::Device, queue: &wgpu::Queue, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
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
            &[255, 255, 255, 255],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
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
    null_texture_bind_group: wgpu::BindGroup,

    draw_command_list: Vec<(wgpu::BindGroup, [u16; 2])>,

    index_count: u32,
}

impl UiRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let null_texture = Texture::from_null(device, queue, "make null texture - STWB");

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

        let null_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("STWB - null bind group layout descriptor"),
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
            })),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&null_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&null_texture.sampler),
                },
            ],
            label: Some("STWB - null texture bind group"),
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
            null_texture_bind_group,
            draw_command_list: Vec::new(),
            index_count: 0,
        }
    }

    pub fn generate_image_bind_group(
        &mut self,
        image: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::BindGroup {
        let diffuse_bytes: &[u8] = &std::fs::read("./src/graphics/".to_owned() + image).unwrap();
        let image_texture =
            Texture::from_bytes(device, queue, diffuse_bytes, "uh buh bu - STWB (who else)");

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&image_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&image_texture.sampler),
                },
            ],
            label: Some("STRWB - texture_bind_group"),
        })
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

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        for (bind_group, indicies) in &self.draw_command_list {
            pass.set_bind_group(1, bind_group, &[]);
            pass.draw_indexed(indicies[0] as u32..indicies[1] as u32, 0, 0..1);
        }
    }

    pub fn image(&mut self, image: &geometry::Image, image_bind_group: wgpu::BindGroup) {
        let vertex_offset = self.vertices.len() as u16;
        let index_offset = self.indices.len() as u16;

        let left = image.x;
        let top = image.y;
        let right = image.x + image.w;
        let bottom = image.y + image.h;

        self.vertices.push(Vertex {
            position: [left, top],
            color: [255, 255, 255, 255],
            uv: [0.0, 0.0],
        });

        self.vertices.push(Vertex {
            position: [right, top],
            color: [255, 255, 255, 255],
            uv: [1.0, 0.0],
        });

        self.vertices.push(Vertex {
            position: [left, bottom],
            color: [255, 255, 255, 255],
            uv: [0.0, 1.0],
        });

        self.vertices.push(Vertex {
            position: [right, bottom],
            color: [255, 255, 255, 255],
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
        self.draw_command_list
            .push((image_bind_group, [index_offset, self.indices.len() as u16]));
    }

    pub fn rect(&mut self, rect: &geometry::Rect) {
        let vertex_offset = self.vertices.len() as u16;
        let index_offset = self.indices.len() as u16;

        let left = rect.x;
        let top = rect.y;
        let right = rect.x + rect.w;
        let bottom = rect.y + rect.h;

        self.vertices.push(Vertex {
            position: [left, top],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [0.0, 0.0],
        });

        self.vertices.push(Vertex {
            position: [right, top],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [0.0, 0.0],
        });

        self.vertices.push(Vertex {
            position: [left, bottom],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [0.0, 0.0],
        });

        self.vertices.push(Vertex {
            position: [right, bottom],
            color: [rect.color.r, rect.color.g, rect.color.b, rect.color.a],
            uv: [0.0, 0.0],
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
        self.draw_command_list.push((
            self.null_texture_bind_group.clone(),
            [index_offset, self.indices.len() as u16],
        ));
    }

    pub fn begin_frame(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.vertices.clear();
        self.indices.clear();
        self.index_count = 0;
        self.draw_command_list.clear();

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
        self.ensure_capacity(device);
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        self.index_count = self.indices.len() as u32;
    }
}
