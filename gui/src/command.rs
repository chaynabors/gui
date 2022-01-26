use std::path::PathBuf;

/// A command type for altering the gui state indirectly.
///
/// This is the preferred means for handling state changes.
#[derive(Clone, Debug)]
pub enum Command<'a> {
    /// A command to redraw the current gui frame.
    Redraw,
    /// A command for resizing the gui.
    Resized {
        ///
        width: u32,
        ///
        height: u32,
    },
    /// A command for when a file has been dropped onto the gui
    ///
    /// It's likely that the easiest way to supply this command is per file.
    /// 
    /// In such a case, it's the preferred method.
    DroppedFile {
        ///
        path: &'a PathBuf,
    },
    /// A command for when a file is being hovered over the gui.
    ///
    /// This will likely be supplied per file being hovered.
    HoveredFile {
        ///
        path: &'a PathBuf,
    },
    /// A command which reverts any hovered file state changes.
    HoveredFileCanceled,
    /// A command to change the scale factor of gui elements.
    ScaleFactorChanged {
        ///
        scale_factor: f64
    },
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
        button: u16,
        ///
        pressed: bool,
    }
}
