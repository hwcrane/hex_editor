use std::io::stdout;

use crossterm::queue;
use crossterm::style::{Print, PrintStyledContent, Stylize};
use crossterm::{cursor::MoveTo, Result};

pub struct Window {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Window {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Window {
        Window {
            x,
            y,
            width,
            height,
        }
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    pub fn set_location(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn move_to(&self, x: u16, y: u16) -> MoveTo {
        MoveTo(&self.x + x, self.y + y)
    }

    pub fn clear(&self) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                queue!(stdout(), self.move_to(x, y), Print(" "))?
            }
        }
        Ok(())
    }

    pub fn fill_red(&self) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                queue!(
                    stdout(),
                    self.move_to(x, y),
                    PrintStyledContent(" ".on_red())
                )?
            }
        }
        Ok(())
    }
}
