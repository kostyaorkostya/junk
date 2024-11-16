mod button;
mod label;
mod window;

use std::result::Result;

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error>;

    /// Draw the widget on standard output.
    fn draw(&self) -> Result<(), std::fmt::Error> {
        let mut buffer = String::new();
        self.draw_into(&mut buffer)?;
        println!("{buffer}");
        Ok(())
    }
}

pub use button::Button;
pub use label::Label;
pub use window::Window;
