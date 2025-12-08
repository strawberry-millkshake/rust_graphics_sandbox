use glfw::{Context, WindowEvent, fail_on_errors};
use crate::graphics_backend::pipeline::build_pipeline;
use glm::*;

mod renderer;
mod graphics_backend;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

struct State<'a>{
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    render_pipeline: wgpu::RenderPipeline,
}

impl State<'_>{

    async fn new<'a>() -> State<'a> {

        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                "hello world",
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
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

        State{
            glfw: glfw,
            window: window,
            device: device,
            queue: queue,
            events: events,
            surface: surface,
            render_pipeline: render_pipeline,
        }

    }

}


fn render(program: &State, cord: Vector2<f32>){

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

    let quad_mesh = graphics_backend::mesh_builder::make_quad(&program.device, cord);

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
        render_pass.set_vertex_buffer(0, quad_mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(quad_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);
    }

    program.queue.submit(Some(encoder.finish()));
    frame.present();

}

fn convert_to_dnc(number: f32, window_size: f32) -> f32{
    ((2.0*number) / window_size) - 1.0
}

async fn run(){

    let mut state = State::new().await;
    let mut cord = Vec2::new(0.5, 0.5);
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;

    while !state.window.should_close() {

        state.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&state.events) {
            match event {
                glfw::WindowEvent::CursorPos(cur_x_pos, cur_y_pos) => {
                    x_pos = convert_to_dnc(cur_x_pos as f32, state.window.get_size().0 as f32);
                    y_pos = convert_to_dnc(cur_y_pos as f32, state.window.get_size().1 as f32);
                },
                glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {
                    cord = Vec2::new(x_pos, -1.0 * y_pos);
                },
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    state.window.set_should_close(true);
                }

                _ => {}
            }
        }

        render(&state, cord);
        state.window.swap_buffers();
    }

}

fn main() {
    pollster::block_on(run());
}