use serde::Deserialize;
use serde::Serialize;

/// Type which represents a dynamic gui position.
///
/// This defaults to `TopLeft`
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    /// Represents a position situated at [-1.0, -1.0]
    pub const TOP_LEFT: Self      = Self { x: -1.0, y: -1.0 };
    /// Represents a position situated at [0.0, -1.0]
    pub const TOP_CENTER: Self    = Self { x:  0.0, y: -1.0 };
    /// Represents a position situated at [1.0, -1.0]
    pub const TOP_RIGHT: Self     = Self { x:  1.0, y: -1.0 };
    /// Represents a position situated at [-1.0, 0.0]
    pub const MIDDLE_LEFT: Self   = Self { x: -1.0, y:  0.0 };
    /// Represents a position situated at [0.0, 0.0]
    pub const MIDDLE_CENTER: Self = Self { x:  0.0, y:  0.0 };
    /// Represents a position situated at [1.0, 0.0]
    pub const MIDDLE_RIGHT: Self  = Self { x:  1.0, y:  0.0 };
    /// Represents a position situated at [-1.0, 1.0]
    pub const BOTTOM_LEFT: Self   = Self { x: -1.0, y:  1.0 };
    /// Represents a position situated at [0.0, 1.0]
    pub const BOTTOM_CENTER: Self = Self { x:  0.0, y:  1.0 };
    /// Represents a position situated at [1.0, 1.0]
    pub const BOTTOM_RIGHT: Self  = Self { x:  1.0, y:  1.0 };

    /// Convenient way to create new positions
    ///
    /// # Arguments
    ///
    /// - `x` - The x value of the position
    ///
    /// - `y` - The y value of the position
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::TOP_LEFT
    }
}
