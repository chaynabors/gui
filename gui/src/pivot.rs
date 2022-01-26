use serde::Deserialize;
use serde::Serialize;

/// Type which represents the pivot of a transform
///
/// This defaults to `TopLeft`
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Pivot {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for Pivot {
    fn default() -> Self {
        Self::TopLeft
    }
}
