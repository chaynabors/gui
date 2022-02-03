use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::ColorType;
use image::GenericImageView;
use image::ImageFormat;
use wgpu::Device;
use wgpu::Extent3d;
use wgpu::Queue;
use wgpu::TextureDescriptor;
use wgpu::TextureDimension;
use wgpu::TextureFormat;
use wgpu::TextureUsages;
use wgpu::TextureViewDescriptor;
use wgpu::util::DeviceExt;

use crate::GuiError;
use crate::GuiResult;

const WHITE_SQUARE: [u8; 16] = [255; 16];

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub dimensions: [u32; 2],
}

impl Texture {
    pub fn from_path<P>(device: &Device, queue: &Queue, path: P) -> GuiResult<Self> where P: AsRef<Path> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let image = image::load(reader, ImageFormat::from_path(&path)?)?;
        let (width, height) = image.dimensions();

        let format = match image.color() {
            ColorType::L8 => TextureFormat::R8Unorm,
            ColorType::La8 => TextureFormat::Rg8Unorm,
            ColorType::Rgba8 => TextureFormat::Rgba8UnormSrgb,
            ColorType::L16 => TextureFormat::R16Uint,
            ColorType::La16 => TextureFormat::Rg16Uint,
            ColorType::Rgba16 => TextureFormat::Rgba16Uint,
            ColorType::Bgra8 => TextureFormat::Bgra8UnormSrgb,
            _ => return Err(GuiError::UnsupportedColorType(image.color())),
        };

        let texture = device.create_texture_with_data(
            queue,
            &TextureDescriptor {
                label: Some(&format!("gui_wgpu {}", path.as_ref().display())),
                size: Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsages::TEXTURE_BINDING,
            },
            image.as_bytes(),
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        Ok(Self {
            texture,
            view,
            dimensions: [width, height],
        })
    }

    pub fn default(device: &Device, queue: &Queue) -> Self {
        let texture = device.create_texture_with_data(
            queue,
            &TextureDescriptor {
                label: Some("gui_wgpu default_texture"),
                size: Extent3d { width: 2, height: 2, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING,
            },
            &WHITE_SQUARE,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        Self {
            texture,
            view,
            dimensions: [2, 2],
        }
    }
}
