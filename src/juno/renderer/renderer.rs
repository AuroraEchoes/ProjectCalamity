use cgmath::Vector2;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::juno::Vertex;

use super::{
    quad::TexturedQuad,
    testing::{TextureAtlasHandle, TextureBank},
    MAX_QUADS,
};

pub struct Renderer {
    interface: WGPUInterface,
    submitted_quads: Vec<TexturedQuad>,
    pub texture_bank: TextureBank,
}

impl Renderer {
    pub fn new(event_loop: &EventLoop<()>, window_size: PhysicalSize<u32>) -> Self {
        let window = WindowBuilder::new()
            .with_inner_size(window_size)
            .build(&event_loop)
            .unwrap();
        let interface = pollster::block_on(WGPUInterface::new(window));
        let submitted_quads = vec![];
        let texture_bank = TextureBank::new();

        Self {
            interface,
            submitted_quads,
            texture_bank,
        }
    }

    pub fn submit_textured_quad(&mut self, quad: TexturedQuad) {
        self.submitted_quads.push(quad);
    }

    pub fn load_atlas(
        &mut self,
        path: &str,
        tile_size_pixels: Vector2<u32>,
    ) -> Option<TextureAtlasHandle> {
        self.texture_bank.load_atlas(
            path,
            tile_size_pixels,
            &mut self.interface.device,
            &mut self.interface.queue,
        )
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. Order quads by texture_id
        self.submitted_quads
            .sort_by(|a, b| a.source().cmp(&b.source()));
        // 2. Store a range of indicies for each texture_id, indexed by texture_id
        let mut indicies = Vec::new();
        let mut curr_count = 0;
        for texture_id in 0..self.texture_bank.next_id() {
            let quads_with_id = self
                .submitted_quads
                .iter()
                .filter(|q| q.source() == texture_id)
                .count();
            let indexes_for_id = quads_with_id as u32 * 6;
            indicies.push(curr_count..(curr_count + indexes_for_id));
            curr_count += indexes_for_id;
        }
        let verticies = self
            .submitted_quads
            .iter_mut()
            .map(|q| q.verticies(&self.interface.window.inner_size()))
            .flatten()
            .collect::<Vec<Vertex>>();

        self.interface.queue.write_buffer(
            &self.interface.vertex_buffer,
            0,
            bytemuck::cast_slice(verticies.as_slice()),
        );

        let frame = self.interface.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.interface
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("juno-command-encoder"),
                });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("juno-render-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.interface.pipeline);
            render_pass.set_vertex_buffer(0, self.interface.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.interface.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            for texture in self.used_textures() {
                let atlas = self.texture_bank.texture(texture);
                render_pass.set_bind_group(0, atlas.bind_group(), &[]);
                render_pass.draw_indexed(indicies[texture as usize].clone(), 0, 0..1);
            }
        }

        self.interface
            .queue
            .submit(std::iter::once(encoder.finish()));
        frame.present();
        self.submitted_quads.clear();

        Ok(())
    }

    fn used_textures(&self) -> Vec<u32> {
        let mut used_textures = Vec::<u32>::new();
        for quad in self.submitted_quads.iter() {
            let source = quad.source();
            if !used_textures.contains(&source) {
                used_textures.push(source)
            }
        }
        used_textures
    }

    fn quads_using_texture(&self, texture: u32) -> Vec<&TexturedQuad> {
        self.submitted_quads
            .iter()
            .filter(|q| q.source() == texture)
            .collect::<Vec<_>>()
    }

    pub fn interface(&mut self) -> &mut WGPUInterface {
        &mut self.interface
    }
}

pub struct WGPUInterface {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: winit::window::Window,
    pub color: wgpu::Color,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl WGPUInterface {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let color = wgpu::Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("Juno Bind Group Layout"),
            });

        let verticies = (0..MAX_QUADS * 4)
            .map(|_| Vertex::default())
            .collect::<Vec<_>>();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(verticies.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // Create index buffer
        // Since all we're doing is drawing quads, this will repeat every 6 indicies
        // Indicies: {0, 1, 2, 2, 3, 0} (note this needs to be offset since we're not drawing
        // the same quad every time). This offset is equal to 4 * MAX_QUADS (4 verticies / quad)
        let indicies = (0..MAX_QUADS)
            .map(|x| [0, 1, 2, 2, 3, 0].map(|y| y + 4 * x))
            .flatten()
            .collect::<Vec<_>>();
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Juno Index Buffer"),
            contents: bytemuck::cast_slice(indicies.as_slice()),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Juno Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            color,
            pipeline,
            vertex_buffer,
            index_buffer,
            bind_group_layout,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn queue_mut(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }

    fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }
}
