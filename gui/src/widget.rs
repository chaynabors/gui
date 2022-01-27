use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

/// A type for the building blocks of a gui application.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Widget {
    /// A window type for grouping widgets.
    Window {
        ///
        label: String,
        ///
        widgets: Vec<Widget>,
    },
    /// A simple text component.
    Text {
        ///
        label: String,
        ///
        text: String,
    },
    /// A pressable button.
    Button {
        ///
        label: String
    },
}

impl Widget {
    /// Get the label of a widget without explicitly pattern matching on its inner type.
    ///
    /// Calling this pattern matches internally.
    pub fn get_label<'a>(&'a self) -> &'a str {
        match self {
            Widget::Window { label, .. } |
            Widget::Text { label, .. } |
            Widget::Button { label } => label
        }
    }
}

impl Display for Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Widget::Window { label, widgets } => {
                writeln!(f, "Begin Window({label})")?;
                for widget in widgets { Display::fmt(widget, f)?; }
                writeln!(f, "End Window({label})")
            }
            Widget::Text { label, .. } => writeln!(f, "Text({label})"),
            Widget::Button { label } => writeln!(f, "Button({label})"),
        }
    }
}
