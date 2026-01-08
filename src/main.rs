use glfw::{Context, WindowEvent, fail_on_errors};
use wgpu::util::DeviceExt;
use crate::graphics_backend::{mesh_builder, pipeline::build_pipeline};
use glam::{Mat4, Vec3};

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
    _config: wgpu::SurfaceConfiguration,
    quad_mesh: mesh_builder::Mesh,
    globals_bind_group: wgpu::BindGroup,
    globals_buffer: wgpu::Buffer
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

        let globals_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("globals bgl - STRWB"),
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

        let globals = mesh_builder::Globals {
            projection: [[0.0; 4]; 4],
            model: [[0.0; 4]; 4],
        };

        let globals_buffer_descriptor = wgpu::util::BufferInitDescriptor {
            label: Some("globals buffer - STRWB"),
            contents: bytemuck::bytes_of(&globals),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        };
        let globals_buffer = device.create_buffer_init(&globals_buffer_descriptor);

        let globals_bind_group_descriptor = wgpu::BindGroupDescriptor {
            label: Some("globals bind group - STRWB"),
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buffer.as_entire_binding(),
            }],
        };

        let globals_bind_group = device.create_bind_group(&globals_bind_group_descriptor);

        let render_pipeline = build_pipeline(&device, &surface, &adapter, &globals_bind_group_layout);

        let quad_mesh = graphics_backend::mesh_builder::make_quad(&device);

        State{
            glfw: glfw,
            window: window,
            device: device,
            queue: queue,
            events: events,
            surface: surface,
            render_pipeline: render_pipeline,
            _config: config,
            quad_mesh: quad_mesh,
            globals_bind_group: globals_bind_group,
            globals_buffer: globals_buffer
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
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.quad_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.quad_mesh.index_count, 0, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

}

fn ui_projection(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh(
        0.0,
        width,
        height,
        0.0,
        -1.0,
        1.0,
    )
}

fn model_for_rect(x: f32, y: f32, w: f32, h: f32) -> Mat4 {
    Mat4::from_scale_rotation_translation(
        Vec3::new(w, h, 1.0),
        glam::Quat::IDENTITY,
        Vec3::new(x, y, 0.0),
    )
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

        let projection = ui_projection(state.window.get_size().0 as f32, state.window.get_size().1 as f32);
        let model = model_for_rect(x_pos, y_pos, 200.0, 120.0);

        let globals = mesh_builder::Globals {
            projection: projection.to_cols_array_2d(),
            model: model.to_cols_array_2d(),
        };

        state.queue.write_buffer(
            &state.globals_buffer,
            0,
            bytemuck::bytes_of(&globals),
        );


        state.queue.write_buffer(&state.globals_buffer, 0, bytemuck::bytes_of(&globals));

        state.render();
    }

}

fn main() {
    pollster::block_on(run());
}