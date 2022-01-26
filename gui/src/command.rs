use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Command<'a> {
    /// A command for removing widgets within a container.
    RemoveWidget {
        ///
        label: &'a str,
    }
}
