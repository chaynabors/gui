use gui::Widget;
use wgpu::BlendState;
use wgpu::ColorTargetState;
use wgpu::ColorWrites;
use wgpu::CommandEncoder;
use wgpu::Device;
use wgpu::FragmentState;
use wgpu::FrontFace;
use wgpu::LoadOp;
use wgpu::MultisampleState;
use wgpu::Operations;
use wgpu::PolygonMode;
use wgpu::PrimitiveState;
use wgpu::PrimitiveTopology;
use wgpu::RenderPassColorAttachment;
use wgpu::RenderPassDescriptor;
use wgpu::RenderPipeline;
use wgpu::RenderPipelineDescriptor;
use wgpu::ShaderModule;
use wgpu::TextureFormat;
use wgpu::TextureView;
use wgpu::VertexState;
use wgpu::include_wgsl;

#[allow(dead_code)]
pub struct Renderer {
    shader: ShaderModule,
    pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Self {
        let shader = device.create_shader_module(&include_wgsl!("gui.wgsl"));

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("gui_wgpu pipeline"),
            layout: None,
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

        Self {
            shader,
            pipeline,
        }
    }

    pub fn render(&self, command_encoder: &mut CommandEncoder, view: &TextureView, gui: &gui::Instance) {
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

        for widget in &gui.widgets {
            match widget {
                Widget::Window { widgets, .. } => {
                    render_pass.set_pipeline(&self.pipeline);
                    render_pass.draw(0..3, 0..1);
                },
                Widget::Text { text, .. } => (),
                Widget::Button { label } => (),
                _ => todo!("Unsupported widget"),
            }
        }
    }
}
