use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub enum Size {
    Fixed { width: u32, height: u32 },
    Dynamic { width: f32, height: f32 },
}

impl Default for Size {
    fn default() -> Self {
        Self::Fixed {
            width: 128,
            height: 64,
        }
    }
}
