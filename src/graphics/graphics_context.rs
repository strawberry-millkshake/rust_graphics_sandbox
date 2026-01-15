use crate::{graphics, ui::Frame};

pub struct Context<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    _config: wgpu::SurfaceConfiguration,
    _surface_format: wgpu::TextureFormat,
    pub ui_renderer: graphics::ui_renderer::UiRenderer,
    ui_pipeline: wgpu::RenderPipeline,
}

impl Context<'_> {
    pub async fn new<'a>(window: &glfw::PWindow) -> Context<'a> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
        let surface = unsafe {
            instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap())
                .unwrap()
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to create adapter - STRWB");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Device - STRWB"),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("Failed to create device - STRWB");

        let config = surface
            .get_default_config(
                &adapter,
                window.get_size().0 as u32,
                window.get_size().1 as u32,
            )
            .unwrap();
        surface.configure(&device, &config);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let (ui_renderer, globals_bind_group_layout) =
            graphics::ui_renderer::UiRenderer::new(&device);

        let ui_pipeline =
            graphics::pipeline::build_pipeline(&device, &globals_bind_group_layout, surface_format);

        Context {
            device: device,
            queue: queue,
            surface: surface,
            _config: config,
            _surface_format: surface_format,
            ui_renderer: ui_renderer,
            ui_pipeline: ui_pipeline,
        }
    }

    pub fn render(&mut self, frame: Frame) {
        self.ui_renderer
            .begin_frame(&self.queue, frame.width, frame.height);

        let _: Vec<_> = frame
            .rec_vec
            .iter()
            .map(|x| self.ui_renderer.rect(x))
            .collect();

        self.ui_renderer.upload(&self.device, &self.queue);

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture - STRWB");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Endcoer - STRWB"),
            });

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

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass - STRWB"),
                color_attachments: &[Some(color_attatchment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.ui_renderer.draw(&mut render_pass, &self.ui_pipeline);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
