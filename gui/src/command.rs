use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Command {
    /// A command which updates the position of the cursor.
    ///
    /// This command must supply the cursor position in physical coordinates.
    CursorMoved {
        ///
        x: u32,
        ///
        y: u32,
    },
    /// A command describing the state of a mouse button
    MouseButton {
        ///
        button: u32,
        ///
        pressed: bool,
    }
}
