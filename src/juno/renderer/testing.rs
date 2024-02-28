use cgmath::Vector2;
use image::GenericImageView;

use super::{quad::TexturedQuad, renderer::WGPUInterface};

pub fn testing(interface: WGPUInterface) {
    let mut texture_bank = TextureBank::new();
    let renderer = Renderer {
        interface,
        submitted_quads: vec![],
        texture_bank,
    };
}

pub struct Renderer {
    interface: WGPUInterface,
    submitted_quads: Vec<TexturedQuad>,
    texture_bank: TextureBank,
}

impl Renderer {
    pub fn new(
        interface: WGPUInterface,
        submitted_quads: Vec<TexturedQuad>,
        texture_bank: TextureBank,
    ) -> Self {
        Self {
            interface,
            submitted_quads,
            texture_bank,
        }
    }

    pub fn submit_textured_quad(&mut self, quad: TexturedQuad) {
        self.submitted_quads.push(quad);
    }

    pub fn load_atlas(&mut self, path: &str, tile_size_pixels: Vector2<u32>) {
        let texture_id = self.texture_bank.next_id();
        let texture_path = format!("assets/{path}");
        if let Ok(texture) = image::open(texture_path.as_str()) {
            let texture_rgba = texture.to_rgba8();
            let (texture_width, texture_height) = texture.dimensions();
            let image_dimensions = Vector2::new(texture_width, texture_height);

            let texture = self
                .interface
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(format!("juno-texture-{}", texture_id).as_str()),
                    size: wgpu::Extent3d {
                        width: texture_width,
                        height: texture_height,
                        depth_or_array_layers: 1,
                    },
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                    mip_level_count: 1,
                    sample_count: 1,
                });

            self.interface.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &texture_rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * texture_width),
                    rows_per_image: Some(texture_height),
                },
                wgpu::Extent3d {
                    width: texture_width,
                    height: texture_height,
                    depth_or_array_layers: 1,
                },
            );

            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = self
                .interface
                .device
                .create_sampler(&wgpu::SamplerDescriptor {
                    label: Some(format!("juno-texture-sampler-{}", texture_id).as_str()),
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                });
            let texture_bind_group_layout =
                self.interface
                    .device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    multisampled: false,
                                    view_dimension: wgpu::TextureViewDimension::D2,
                                    sample_type: wgpu::TextureSampleType::Float {
                                        filterable: true,
                                    },
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
                        label: Some(format!("juno-bind-group-layout-{}", texture_id).as_str()),
                    });

            let bind_group = self
                .interface
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(format!("juno-bind-group-{}", texture_id).as_str()),
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                });
            let texture_atlas =
                TextureAtlasBackend::new(texture, bind_group, tile_size_pixels, image_dimensions);
        } else {
            println!("[ERROR]: Unable to load texture at {}", path);
        }
    }
}

pub struct TextureAtlasBackend {
    texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
    // Tile dimensions in clip space
    tile_size: Vector2<f32>,
    // Dimensions of texture atlas in number of tiles
    atlas_tiles_size: Vector2<u32>,
    atlas_pixels_size: Vector2<u32>,
}

impl TextureAtlasBackend {
    pub fn new(
        texture: wgpu::Texture,
        bind_group: wgpu::BindGroup,
        tile_pixel_size: Vector2<u32>,
        image_dimensions: Vector2<u32>,
    ) -> Self {
        let atlas_tiles_size = image_dimensions.zip(tile_pixel_size, |a, b| a / b);
        let tile_pixel_size_f32 = tile_pixel_size
            .cast::<f32>()
            .unwrap_or(Vector2::new(1., 1.));
        let image_dimensions_f32 = image_dimensions
            .cast::<f32>()
            .unwrap_or(Vector2::new(1., 1.));
        let tile_clip_size = tile_pixel_size_f32.zip(image_dimensions_f32, |a, b| a / b);
        Self {
            texture,
            bind_group,
            tile_size: tile_clip_size,
            atlas_tiles_size,
            atlas_pixels_size: image_dimensions,
        }
    }

    pub fn atlas_tiles_size(&self) -> &Vector2<u32> {
        &self.atlas_tiles_size
    }

    pub fn clip_tile_size(&self) -> &Vector2<f32> {
        &self.tile_size
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

pub struct TextureAtlasHandle {
    source: u32,
    tile_size: Vector2<f32>,
}

impl TextureAtlasHandle {
    pub fn texture(&self, x: u32, y: u32) -> TextureSection {
        TextureSection {
            source_texture: self.source,
            position: Vector2::new(x as f32 * self.tile_size.x, y as f32 * self.tile_size.y),
            size: self.tile_size,
        }
    }

    pub fn new(tile_size: Vector2<f32>, source: u32) -> Self {
        Self { source, tile_size }
    }
}

#[derive(Debug)]
pub struct TextureSection {
    source_texture: u32,
    position: Vector2<f32>,
    size: Vector2<f32>,
}

impl TextureSection {
    pub fn new(source_texture: u32, position: Vector2<f32>, size: Vector2<f32>) -> Self {
        Self {
            source_texture,
            position,
            size,
        }
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn source(&self) -> u32 {
        self.source_texture
    }
}

pub struct TextureBank {
    pub textures: Vec<TextureAtlasBackend>,
}

impl TextureBank {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }

    pub fn next_id(&self) -> u32 {
        self.textures.len() as u32
    }

    pub fn load_atlas(
        &mut self,
        path: &str,
        tile_size_pixels: Vector2<u32>,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
    ) -> Option<TextureAtlasHandle> {
        let texture_id = self.next_id();
        let texture_path = format!("assets/{path}");
        if let Ok(texture) = image::open(texture_path.as_str()) {
            let texture_rgba = texture.to_rgba8();
            let (texture_width, texture_height) = texture.dimensions();
            let image_dimensions = Vector2::new(texture_width, texture_height);

            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(format!("juno-texture-{}", texture_id).as_str()),
                size: wgpu::Extent3d {
                    width: texture_width,
                    height: texture_height,
                    depth_or_array_layers: 1,
                },
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
                mip_level_count: 1,
                sample_count: 1,
            });

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &texture_rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * texture_width),
                    rows_per_image: Some(texture_height),
                },
                wgpu::Extent3d {
                    width: texture_width,
                    height: texture_height,
                    depth_or_array_layers: 1,
                },
            );

            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some(format!("juno-texture-sampler-{}", texture_id).as_str()),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
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
                    label: Some(format!("juno-bind-group-layout-{}", texture_id).as_str()),
                });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(format!("juno-bind-group-{}", texture_id).as_str()),
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });
            let texture_atlas =
                TextureAtlasBackend::new(texture, bind_group, tile_size_pixels, image_dimensions);
            let handle = Some(TextureAtlasHandle::new(texture_atlas.tile_size, texture_id));

            self.textures.push(texture_atlas);
            handle
        } else {
            println!("[ERROR]: Unable to load texture at {}", path);
            None
        }
    }

    pub fn texture(&self, id: u32) -> &TextureAtlasBackend {
        &self.textures[id as usize]
    }
}
