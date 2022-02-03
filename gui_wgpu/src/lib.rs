mod gui_error;
mod renderer;
mod texture;

use texture::Texture;

pub use gui_error::GuiError;
pub use renderer::Renderer;

pub type GuiResult<T> = Result<T, GuiError>;
