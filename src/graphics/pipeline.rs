use crate::graphics::ui_renderer::Vertex;
use std::env::current_dir;
use std::fs;

fn get_shader_file() -> String {
    let mut filepath = current_dir().unwrap();
    filepath.push("src/shaders/shader.wgsl");
    let filepath = filepath.into_os_string().into_string().unwrap();
    fs::read_to_string(filepath).expect("can't read source code - STRWB")
}

pub fn build_pipeline(
    device: &wgpu::Device,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
    surface_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
        label: Some("Shader Module - STRWB"),
        source: wgpu::ShaderSource::Wgsl(get_shader_file().into()),
    };

    let shader_module = device.create_shader_module(shader_module_descriptor);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("ui pipeline layout - STRWB"),
        bind_group_layouts: &[&uniform_bind_group_layout, texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("ui pipeline - STRWB"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::get_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}
