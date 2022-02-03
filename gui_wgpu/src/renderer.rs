use std::collections::HashMap;
use std::path::PathBuf;

use bytemuck::Pod;
use bytemuck::Zeroable;
use gui::Container;
use gui::Size;
use gui::View;
use wgpu::BindGroup;
use wgpu::BindGroupDescriptor;
use wgpu::BindGroupEntry;
use wgpu::BindGroupLayout;
use wgpu::BindGroupLayoutDescriptor;
use wgpu::BindGroupLayoutEntry;
use wgpu::BindingResource;
use wgpu::BindingType;
use wgpu::BlendState;
use wgpu::Buffer;
use wgpu::BufferAddress;
use wgpu::BufferBinding;
use wgpu::BufferBindingType;
use wgpu::BufferDescriptor;
use wgpu::BufferSize;
use wgpu::BufferUsages;
use wgpu::ColorTargetState;
use wgpu::ColorWrites;
use wgpu::CommandEncoder;
use wgpu::Device;
use wgpu::DynamicOffset;
use wgpu::FragmentState;
use wgpu::FrontFace;
use wgpu::LoadOp;
use wgpu::MultisampleState;
use wgpu::Operations;
use wgpu::PipelineLayoutDescriptor;
use wgpu::PolygonMode;
use wgpu::PrimitiveState;
use wgpu::PrimitiveTopology;
use wgpu::Queue;
use wgpu::RenderPassColorAttachment;
use wgpu::RenderPassDescriptor;
use wgpu::RenderPipeline;
use wgpu::RenderPipelineDescriptor;
use wgpu::ShaderModule;
use wgpu::ShaderStages;
use wgpu::TextureFormat;
use wgpu::TextureSampleType;
use wgpu::TextureView;
use wgpu::TextureViewDimension;
use wgpu::VertexState;
use wgpu::include_wgsl;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;

use crate::GuiResult;
use crate::Texture;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Globals {
    resolution: [u32; 2],
    scale_factor: f32,
    _pad: u32,
}

impl Globals {
    fn new(resolution: [u32; 2], scale_factor: f64) -> Self {
        Self {
            resolution,
            scale_factor: scale_factor as f32,
            _pad: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ContainerLocals {
    tex_coord: [i32; 2],
    color: [u8; 4],
    _pad: [u8; 128],
    _pad2: [u8; 64],
    _pad3: [u8; 32],
    _pad4: [u8; 20],
}

impl ContainerLocals {
    fn new(tex_coord: [i32; 2], color: [u8; 4]) -> Self {
        Self {
            tex_coord,
            color,
            _pad: [0; 128],
            _pad2: [0; 64],
            _pad3: [0; 32],
            _pad4: [0; 20],
        }
    }
}

#[allow(dead_code)]
pub struct Renderer {
    resolution: [u32; 2],
    scale_factor: f64,
    shader: ShaderModule,
    container_bind_group_layout: BindGroupLayout,
    pipeline: RenderPipeline,
    default_texture: Texture,
    textures: HashMap<PathBuf, Texture>,
    globals: Buffer,
    container_locals: Buffer,
    container_bind_groups: Vec<BindGroup>,
}

impl Renderer {
    pub fn from_gui(
        device: &Device,
        queue: &Queue,
        texture_format: TextureFormat,
        resolution: [u32; 2],
        scale_factor: f64,
        containers: &[Container],
    ) -> GuiResult<Self> {
        let shader = device.create_shader_module(&include_wgsl!("container.wgsl"));

        let container_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("gui_wgpu container_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(std::mem::size_of::<Globals>() as _),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: BufferSize::new(std::mem::size_of::<ContainerLocals>() as _),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("gui_wgpu pipeline_layout"),
            bind_group_layouts: &[&container_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("gui_wgpu pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[ColorTargetState {
                    format: texture_format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                }],
            }),
            multiview: None,
        });

        let default_texture = Texture::default(&device, &queue);
        let mut textures = HashMap::new();

        let globals = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("gui_wgpu globals"),
            contents: bytemuck::bytes_of(&Globals::new(resolution, scale_factor)),
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });

        let container_locals = device.create_buffer(&BufferDescriptor {
            label: Some("gui_wgpu container_locals"),
            size: (containers.len() * std::mem::size_of::<ContainerLocals>()) as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let mut container_bind_groups = vec![];
        for i in 0..containers.len() {
            let position = {
                let [mut x, mut y] = containers[i].screen_position;
                x = x * resolution[0] as f32 + containers[i].pixel_position[0] as f32;
                y = y * resolution[1] as f32 + containers[i].pixel_position[1] as f32;
                [x as i32, y as i32]
            };

            let (color, texture) = match &containers[i].view {
                View::Simple { color, .. } => (*color, &default_texture),
                View::Texture { path, color } => {
                    let texture = match textures.get(path) {
                        Some(texture) => texture,
                        None => {
                            let texture = Texture::from_path(device, queue, path)?;
                            textures.insert(path.to_path_buf(), texture);
                            textures.get(path).unwrap()
                        },
                    };

                    (color.unwrap_or([255; 4]), texture)
                },
                View::CellTexture { path, color, .. } => {
                    let texture = match textures.get(path) {
                        Some(texture) => texture,
                        None => {
                            let texture = Texture::from_path(device, queue, path)?;
                            textures.insert(path.to_path_buf(), texture);
                            textures.get(path).unwrap()
                        },
                    };

                    (color.unwrap_or([255; 4]), texture)
                }
            };

            // write container locals
            queue.write_buffer(
                &container_locals,
                i as BufferAddress * std::mem::size_of::<ContainerLocals>() as BufferAddress,
                bytemuck::bytes_of(&ContainerLocals::new(position, color)),
            );

            // create bind group per container
            container_bind_groups.push(device.create_bind_group(&BindGroupDescriptor {
                label: Some(&format!("gui_wgpu bind_group {i}")),
                layout: &container_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: globals.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Buffer(BufferBinding {
                            buffer: &container_locals,
                            offset: 0,
                            size: BufferSize::new(std::mem::size_of::<ContainerLocals>() as _),
                        }),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(&texture.view),
                    },
                ],
            }));
        }

        Ok(Self {
            resolution,
            scale_factor,
            shader,
            container_bind_group_layout,
            pipeline,
            default_texture,
            textures,
            globals,
            container_locals,
            container_bind_groups,
        })
    }

    pub fn resize(&mut self, queue: &Queue, resolution: [u32; 2], scale_factor: f64, containers: &[Container]) {
        self.resolution = resolution;
        self.scale_factor = scale_factor;

        // write globals
        queue.write_buffer(&self.globals, 0, bytemuck::bytes_of(&Globals::new(resolution, scale_factor)));

        for i in 0..containers.len() {
            let [width, height] = self.container_size(&containers[i]);
            let position = self.container_position(&containers[i], width, height);

            let color = match &containers[i].view {
                View::Simple { color, .. } => (*color),
                View::Texture { color, .. } => color.unwrap_or([255; 4]),
                View::CellTexture { color, .. } => color.unwrap_or([255; 4]),
            };

            // write container locals
            queue.write_buffer(
                &self.container_locals,
                i as BufferAddress * std::mem::size_of::<ContainerLocals>() as BufferAddress,
                bytemuck::bytes_of(&ContainerLocals::new(position, color)),
            );
        }
    }

    pub fn render(
        &self,
        command_encoder: &mut CommandEncoder,
        view: &TextureView,
        containers: &[Container],
    ) {
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("gui_wgpu render_pass"),
            color_attachments: &[RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        for i in 0..containers.len() {
            let [width, height] = self.container_size(&containers[i]);
            let [x, y] = self.container_position(&containers[i], width, height);

            // validate
            if width == 0 || height == 0 { continue; }
            if x + (width as i32) < 0 || x >= self.resolution[0] as i32 { continue; }
            if y + (height as i32) < 0 || y >= self.resolution[1] as i32 { continue; }

            // clamp to view for scissor
            let x = x.max(0) as u32;
            let y = y.max(0) as u32;
            let width = width.min(self.resolution[0] - x);
            let height = height.min(self.resolution[1] - y);

            // draw the container
            render_pass.set_scissor_rect(x, y, width, height);
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.container_bind_groups[i], &[(i * std::mem::size_of::<ContainerLocals>()) as DynamicOffset]);
            render_pass.draw(0..3, 0..1);
        }
    }

    fn container_size(&self, container: &Container) -> [u32; 2] {
        match &container.view {
            View::CellTexture { width, height, .. } | View::Simple { width, height, .. } => [
                match width {
                    Size::Fixed(size) => (*size as f64 * self.scale_factor) as u32,
                    Size::Dynamic(size) => (size * self.resolution[0] as f32) as u32,
                },
                match height {
                    Size::Fixed(size) => (*size as f64 * self.scale_factor) as u32,
                    Size::Dynamic(size) => (size * self.resolution[1] as f32) as u32,
                },
            ],
            View::Texture { path, .. } => {
                let texture = self.textures.get(path).unwrap();
                let width = (texture.dimensions[0] as f64 * self.scale_factor) as u32;
                let height = (texture.dimensions[1] as f64 * self.scale_factor) as u32;
                [width, height]
            },
        }
    }

    fn container_position(&self, container: &Container, width: u32, height: u32) -> [i32; 2] {
        let [mut x, mut y] = container.screen_position;
        x = x * self.resolution[0] as f32 + container.pixel_position[0] as f32;
        y = y * self.resolution[1] as f32 + container.pixel_position[1] as f32;
        x -= width as f32 * container.pivot[0];
        y -= height as f32 * container.pivot[1];
        [x as i32, y as i32]
    }
}
