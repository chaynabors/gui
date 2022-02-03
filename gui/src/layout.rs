use serde::Deserialize;
use serde::Serialize;

use crate::Widget;

#[derive(Debug, Deserialize, Serialize)]
pub enum LayoutPadding {
    Static(u32),
    Dynamic(f32),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Layout {
    Free(Widget),
    Vertical {
        ///
        widgets: Vec<Widget>,
        ///
        padding: LayoutPadding,
    },
    Horizontal {
        ///
        widgets: Vec<Widget>,
        ///
        padding: LayoutPadding,
    },
}
