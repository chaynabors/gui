use std::collections::VecDeque;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use crate::Size;
use crate::command::Command;
use crate::event::Event;
use crate::widget::Widget;

/// This type acts as a storage for the top level gui state.
///
/// It is also a converging point for widget events
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Instance {
    widgets: Vec<Widget>,
    #[serde(skip)]
    event_queue: VecDeque<Event>,
    #[serde(skip)]
    resolution: Size,
    #[serde(skip)]
    scale_factor: f64,
}

impl Instance {
    /// Contruct a new default instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a widget to the instance.
    ///
    /// # Arguments
    ///
    /// - `widget` - The widget to add to the instance
    pub fn add_widget<'a>(&'a mut self, widget: Widget) -> &'a mut Self {
        self.widgets.push(widget);
        self
    }

    /// Remove a widget from the instance.
    ///
    /// # Arguments
    ///
    /// - `label` - The label of the widget to remove
    pub fn remove_widget<'a>(&'a mut self, label: &'a str) -> &'a mut Self {
        self.widgets.retain(|w| w.get_label() != label);
        self
    }

    /// Process the given command.
    ///
    /// # Arguments
    ///
    /// - `command` - The command to process
    pub fn process_command(&mut self, command: Command) {
        match command {
            Command::Resized { width, height } => self.resolution = Size::Fixed { width, height },
            Command::ScaleFactorChanged { scale_factor } => self.scale_factor = scale_factor,
            _ => todo!("Unhandled command: {command:?}"),
        }
    }

    /// Retrieve an event from the internal event buffer.
    ///
    /// If there are no events, this will return `Option::None`
    pub fn get_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Begin Instance")?;
        for widget in &self.widgets { Display::fmt(widget, f)?; }
        writeln!(f, "End Instance")
    }
}

