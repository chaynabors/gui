use serde::Deserialize;
use serde::Serialize;

use crate::Event;
use crate::Layout;
use crate::View;

/// The primary container type which defines how widgets are positioned.
#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    /// Position of the container in screen space coordinates.
    ///
    /// The top left corner is `[0.0, 0.0]`
    ///
    /// The bottom right corner is `[1.0, 1.0]`
    pub screen_position: [f32; 2],
    /// Position of the container in pixel coordinates.
    ///
    /// The top left corner is `[0, 0]`
    ///
    /// The bottom right corner is `[width, height]`
    pub pixel_position: [i32; 2],
    /// Pivot of the containers position.
    ///
    /// To pivot off the top left corner of the container, use `[0.0, 0.0]`
    ///
    /// To pivot off the bottom right corner of the container, use `[1.0, 1.0]`
    pub pivot: [f32; 2],
    pub view: View,
    pub layout: Layout,
}

impl Container {
    /// Process the given event.
    ///
    /// # Arguments
    ///
    /// - `event` - The event to process
    pub fn process_event(&mut self, event: Event) {
        match event {
            _ => todo!("Unhandled command: {event:?}"),
        }
    }
}
