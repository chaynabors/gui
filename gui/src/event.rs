use serde::Deserialize;
use serde::Serialize;

/// A type for describing emitted events.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Event {
    /// Emitted when a button has been pressed.
    ButtonPressed {
        ///
        label: String,
    },
    /// Emitted when the button comes into focus
    /// either by the cursor hovering over it or it being selected by some other means
    /// e.g. a controller navigates to the button using a cursor.
    ButtonFocused {
        ///
        label: String,
    },
    /// Emitted when the button comes out of focus
    /// either by the cursor hovering off of it or it being deselected by some other means
    /// e.g. a controller navigates somewhere else.
    ButtonUnfocused {
        ///
        label: String,
    },
    /// Emitted when a button has been released.
    ButtonReleased {
        ///
        label: String,
    },
}
