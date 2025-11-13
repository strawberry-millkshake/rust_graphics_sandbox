use glfw::{Context, fail_on_errors};
use crate::graphics_backend::pipeline::build_pipeline;

mod graphics_backend;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

struct Program<'a>{
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    render_pipeline: wgpu::RenderPipeline,
    triangle_mesh: wgpu::Buffer,
}

async fn setup<'a>() -> Program<'a> {

    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, _events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            "hello world",
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    window.make_current();

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
    let surface= unsafe{instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap()).unwrap()};

    let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    };
    let adapter = instance.request_adapter(&adapter_descriptor).await.expect("Failed to create adapter - STRWB");

    let device_descriptor = wgpu::DeviceDescriptor {
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        label: Some("Device - STRWB"),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
    };
    let (device, queue) = adapter.request_device(&device_descriptor).await.expect("Failed to create device - STRWB");

    let config = surface
        .get_default_config(&adapter, window.get_size().0 as u32, window.get_size().0 as u32)
        .unwrap();
    surface.configure(&device, &config);

    let render_pipeline = build_pipeline(&device, &surface, &adapter);

    let triangle_mesh = graphics_backend::mesh_builder::make_triangle(&device);

    Program{
        glfw: glfw,
        window: window,
        device: device,
        queue: queue,
        surface: surface,
        render_pipeline: render_pipeline,
        triangle_mesh: triangle_mesh,
    }

}

fn render(program: &Program){

    let frame = program.surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture - STRWB");

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let command_encoder_description = wgpu::CommandEncoderDescriptor{
        label: Some("Command Endcoer - STRWB"),
    };
    let mut encoder = program.device.create_command_encoder(&command_encoder_description);

    {
        let color_attatchment = wgpu::RenderPassColorAttachment {
            view: &view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        };
        let render_pass_descriptor = wgpu::RenderPassDescriptor{
            label: Some("Render Pass - STRWB"),
            color_attachments: &[Some(color_attatchment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);
        render_pass.set_pipeline(&program.render_pipeline);
        render_pass.set_vertex_buffer(0, program.triangle_mesh.slice(..));
        render_pass.draw(0..3, 0..1);
    }

    program.queue.submit(Some(encoder.finish()));
    frame.present();

}

async fn run(){

    let mut program = setup().await;

    while !program.window.should_close() {

        program.glfw.poll_events();
        render(&program);
        program.window.swap_buffers();
    }

}

fn main() {
    pollster::block_on(run());
}