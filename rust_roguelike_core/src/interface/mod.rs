pub mod input;
pub mod rendering;

use input::{KeyCode, MouseButton};
use rendering::Renderer;

/// A trait to handle simple applications like the examples.
pub trait App {
    /// Initializes the application.
    ///
    /// Can be used to load textures or prepare other data.
    /// The `Renderer` should not be used to render anything.
    fn init(&mut self, _renderer: &mut dyn Renderer) {}

    /// Renders the application.
    fn render(&mut self, renderer: &mut dyn Renderer);

    /// Handles keyboard input
    fn on_key_released(&mut self, _key: KeyCode) {}

    /// Handles mouse input
    fn on_button_released(&mut self, _button: MouseButton, _index: usize) {}
}
