use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Size {
    Fixed(u32),
    Dynamic(f32),
}
