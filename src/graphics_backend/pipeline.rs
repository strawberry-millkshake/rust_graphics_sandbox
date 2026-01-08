use std::env::current_dir;
use std::fs;

use crate::graphics_backend::mesh_builder;

fn get_shader_file() -> String{
    let mut filepath = current_dir().unwrap();
    filepath.push("src/shaders/shader.wgsl");
    let filepath = filepath.into_os_string().into_string().unwrap();
    fs::read_to_string(filepath).expect("can't read source code - STRWB")
}

pub fn build_pipeline(device: &wgpu::Device, surface: &wgpu::Surface, adapter: &wgpu::Adapter, globals_bind_group_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {

    let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
        label: Some("Shader Module - STRWB"),
        source: wgpu::ShaderSource::Wgsl(get_shader_file().into()),
    };
    let shader_module = device.create_shader_module(shader_module_descriptor);

    let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor{
        label: Some("render pipleline layout - STRWB"),
        bind_group_layouts: &[globals_bind_group_layout],
        push_constant_ranges: &[],
    };
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let layout = mesh_builder::Vertex::get_layout();
    let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {

        label: Some("Render Pipeline - STRWB"),
        layout: Some(&pipeline_layout),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multiview: None,
        multisample: wgpu::MultisampleState::default(),
        cache: None,

        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[layout],
            compilation_options: Default::default(),
        },

        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: swapchain_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
    };
    
    device.create_render_pipeline(&render_pipeline_descriptor)

}