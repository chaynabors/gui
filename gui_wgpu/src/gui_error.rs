use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum GuiError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
    UnsupportedColorType(image::ColorType),
}

impl Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f),
            Self::ImageError(e) => e.fmt(f),
            Self::UnsupportedColorType(color_type) => writeln!(f, "unsupported color type: {color_type:?}")
        }
    }
}

impl Error for GuiError {}

impl From<std::io::Error> for GuiError {
    fn from(from: std::io::Error) -> Self {
        Self::IoError(from)
    }
}

impl From<image::ImageError> for GuiError {
    fn from(from: image::ImageError) -> Self {
        Self::ImageError(from)
    }
}
