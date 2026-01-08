use glfw::{Context, WindowEvent, fail_on_errors};
use crate::graphics_backend::{ui_renderer::UiRenderer, ui_renderer::Rect};

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
    _config: wgpu::SurfaceConfiguration,
    ui: UiRenderer,
    ui_pipeline: wgpu::RenderPipeline,
    _surface_format: wgpu::TextureFormat,
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
            .get_default_config(&adapter, window.get_size().0 as u32, window.get_size().1 as u32)
            .unwrap();
        surface.configure(&device, &config);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let (ui, globals_bind_group_layout) = UiRenderer::new(&device);

        let ui_pipeline = graphics_backend::pipeline::build_pipeline(&device, &globals_bind_group_layout, surface_format);

        State{
            glfw: glfw,
            window: window,
            device: device,
            queue: queue,
            events: events,
            surface: surface,
            _config: config,
            ui: ui,
            ui_pipeline: ui_pipeline,
            _surface_format: surface_format,
        }

    }

    fn render(&mut self){
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture - STRWB");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder_description = wgpu::CommandEncoderDescriptor{
            label: Some("Command Endcoer - STRWB"),
        };
        let mut encoder = self.device.create_command_encoder(&command_encoder_description);

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

            self.ui.draw(&mut render_pass, &self.ui_pipeline);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

}

async fn run(){

    let mut state = State::new().await;
    let mut x_pos: f32 = 0.0;
    let mut y_pos: f32 = 0.0;

    while !state.window.should_close() {

        state.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&state.events) {
            match event {
                glfw::WindowEvent::CursorPos(cur_x_pos, cur_y_pos) => {
                    x_pos = cur_x_pos as f32;
                    y_pos = cur_y_pos as f32;
                },
                glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {
                },
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    state.window.set_should_close(true);
                }

                _ => {}
            }
        }

        let (w, h) = state.window.get_size();
        state.ui.begin_frame(&state.queue, w as u32, h as u32);

        // Example: draw a draggable rect and a header bar
        state.ui.rect(Rect { x: 20.0, y: 20.0, w: 300.0, h: 40.0 }, [0.2, 0.2, 0.2, 1.0]);
        state.ui.rect(Rect { x: x_pos, y: y_pos, w: 120.0, h: 80.0 }, [0.7, 0.2, 0.2, 0.9]);

        state.ui.upload(&state.device, &state.queue);

        state.render();
    }

}

fn main() {
    pollster::block_on(run());
}