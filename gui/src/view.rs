use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::Size;

#[derive(Debug, Deserialize, Serialize)]
pub enum View {
    Simple {
        ///
        width: Size,
        ///
        height: Size,
        ///
        color: [u8; 4],
    },
    Texture {
        ///
        path: PathBuf,
        ///
        color: Option<[u8; 4]>,
    },
    CellTexture {
        ///
        path: PathBuf,
        ///
        width: Size,
        ///
        height: Size,
        ///
        cell_size: [u32; 2],
        ///
        color: Option<[u8; 4]>,
    },
}

impl Default for View {
    fn default() -> Self {
        Self::Simple {
            width: Size::Fixed(426),
            height: Size::Fixed(240),
            color: [255, 255, 255, 255],
        }
    }
}
