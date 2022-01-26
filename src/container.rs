use std::fmt::Debug;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use crate::command::Command;
use crate::pivot::Pivot;
use crate::size::Size;
use crate::widget;
use crate::widget::Widget;

#[derive(Debug, Deserialize, Serialize)]
struct Transform {
    position: [f32; 2],
    size: Size,
    pivot: Pivot,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.2, 0.2],
            size: Size::default(),
            pivot: Pivot::default(),
        }
    }
}

/// Container for widgets.
///
/// This defaults to a fullscreen container but can be given a size and position.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Container {
    label: String,
    transform: Option<Transform>,
    widgets: Vec<Widget>,
}

impl Container {
    /// Construct a new instance of a container.
    ///
    /// This function will return a fullscreen container but can be given a size and position.
    ///
    /// # Arguments
    ///
    /// - `label` - A unique identifier for the container which must be unique among all containers
    pub fn new<'a>(label: &'a str) -> Self {
        Self {
            label: label.to_string(),
            ..Default::default()
        }
    }

    /// Set the position of the container.
    ///
    /// Calling this on a fullscreen container will transform it into a windowed container.
    ///
    /// # Arguments
    ///
    /// - `position` - The position of the container
    pub fn set_position<'a>(&'a mut self, position: [f32; 2]) -> &'a mut Self {
        match self.transform {
            Some(ref mut transform) => transform.position = position,
            None => self.transform = Some(Transform {
                position,
                ..Default::default()
            }),
        };
        self
    }

    /// Set the size of the container.
    ///
    /// Calling this on a fullscreen container will transform it into a windowed container.
    ///
    /// # Arguments
    ///
    /// - `size` - The size of the container
    pub fn set_size<'a>(&'a mut self, size: Size) -> &'a mut Self {
        match self.transform {
            Some(ref mut transform) => transform.size = size,
            None => self.transform = Some(Transform {
                size,
                ..Default::default()
            }),
        };
        self
    }

    /// Set the pivot of a container.
    ///
    /// Calling this on a fullscreen container will transform it into a windowed container.
    ///
    /// # Arguments
    ///
    /// - `pivot` - The pivot of the container
    pub fn set_pivot<'a>(&'a mut self, pivot: Pivot) -> &'a mut Self {
        match self.transform {
            Some(ref mut transform) => transform.pivot = pivot,
            None => self.transform = Some(Transform {
                pivot,
                ..Default::default()
            }),
        };
        self
    }

    /// Make the container fullscreen by stripping its position and size.
    ///
    /// Calling this on a fullscreen container does nothing.
    pub fn make_fullscreen<'a>(&'a mut self) -> &'a mut Self {
        self.transform = None;
        self
    }

    /// Add a widget to the container.
    ///
    /// # Arguments
    ///
    /// - `widget` - The widget to add to the container
    pub fn add_widget<'a>(&'a mut self, widget: Widget) -> &'a mut Self {
        self.widgets.push(widget);
        self
    }

    /// Process a command for the container and its children.
    ///
    /// # Arguments
    ///
    /// - `command` - The command to propogate to child widgets
    pub fn process_command<'a>(&'a mut self, command: Command) -> &'a mut Self {
        match command {
            Command::RemoveWidget { label } => self.widgets.retain(|w| w.get_label() != label ),
        }
        self
    }

    /// Closes the container.
    ///
    /// This is nonreversible.
    pub fn close(self) {
        // This is deliberately empty since we consume self
    }
}

impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Container({}):", self.label)?;
        for widget in &self.widgets {
            f.write_str("    ")?;
            Display::fmt(widget, f)?;
        }
        Ok(())
    }
}
