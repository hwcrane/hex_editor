use crate::hex_file::HexFile;

mod byte;
mod hex_file;
mod window;

use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::queue;
use crossterm::style::{Print, PrintStyledContent, Stylize};
use crossterm::terminal::{size, Clear};
use crossterm::{
    event::{poll, read, Event, KeyCode},
    style, Result,
};
use window::Window;

#[derive(Clone)]
struct Position {
    index: usize,
    first_half: bool,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

struct FileView {
    pub rows: u16,
    pub columns: u16,
    pub starting_row: u16,
}

struct App {
    file: HexFile,
    view: FileView,
    offset_window: Window,
    hex_window: Window,
    char_window: Window,
    cursor_pos: Position,
    width: u16,
    height: u16,
}

impl App {
    pub fn new(file: HexFile) -> App {
        let (width, height) = size().unwrap();
        let columns = ((width - 10) / 4) - 1;
        let rows = height;
        let view = FileView {
            rows,
            columns,
            starting_row: 0,
        };
        let offset_window = Window::new(0, 0, 8, height);
        let hex_window = Window::new(11, 0, columns * 3, height);
        let char_window = Window::new(width - columns - 1, 0, columns, height);
        let cursor_pos = Position {
            index: 0,
            first_half: true,
        };

        App {
            file,
            offset_window,
            hex_window,
            char_window,
            view,
            cursor_pos,
            width,
            height,
        }
    }

    fn resize(&mut self, height: u16, width: u16) {
        queue!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
        self.width = width;
        self.height = height;
        self.view.columns = ((width - 10) / 4) - 1;
        self.view.rows = height;

        // let offset_window = Window::new(0, 0, 8, height);
        // let hex_window = Window::new(10, 0, columns * 3, height);
        // let char_window = Window::new(width - columns - 1, 0, columns, height);

        self.offset_window.set_height(self.view.rows);
        self.hex_window.set_height(self.view.rows);
        self.char_window.set_height(self.view.rows);
        self.hex_window.set_width(self.view.columns * 3);
        self.char_window.set_width(self.view.columns);
        self.char_window
            .set_location(self.width - self.view.columns - 1, 0);
        self.redraw_windows().unwrap();
    }
    fn scroll_down(&mut self) -> Result<()> {
        self.view.starting_row += 1;
        self.redraw_windows()?;
        Ok(())
    }

    fn scroll_up(&mut self) -> Result<()> {
        if self.view.starting_row > 0 {
            self.view.starting_row -= 1;
            self.redraw_windows()?;
        }
        Ok(())
    }

    fn redraw_windows(&mut self) -> Result<()> {
        queue!(stdout(), Hide)?;
        // self.hex_window.clear()?;
        // self.char_window.clear()?;
        // self.offset_window.clear()?;
        self.draw_offset()?;
        self.draw_char()?;
        self.draw_hex()?;
        self.move_cursor(Direction::None);
        queue!(stdout(), Show)?;
        Ok(())
    }

    fn draw_offset(&self) -> Result<()> {
        let start_point = self.view.starting_row * self.view.columns;
        let end_point =
            (start_point + (self.view.rows * self.view.columns)).min(self.file.length as u16);

        for i in (start_point..end_point).step_by(self.view.columns as usize) {
            queue!(
                stdout(),
                self.offset_window
                    .move_to(0, (i - start_point) as u16 / self.view.columns),
                PrintStyledContent(format!("{:08X}", i).black())
            )?;
        }
        Ok(())
    }

    fn move_cursor(&mut self, direction: Direction) {
        let old_position = self.cursor_pos.clone();
        match direction {
            Direction::Up => {
                // Check if already on top row
                if self.cursor_pos.index >= self.view.columns as usize {
                    self.cursor_pos.index -= self.view.columns as usize;
                }
            }
            Direction::Left => {
                // If in first half move to previous
                if self.cursor_pos.first_half {
                    if (self.cursor_pos.index % self.view.columns as usize) != 0 {
                        self.cursor_pos.index -= 1;
                        self.cursor_pos.first_half = false;
                    }
                } else {
                    self.cursor_pos.first_half = true;
                }
            }
            Direction::Right => {
                if self.cursor_pos.first_half {
                    self.cursor_pos.first_half = false;
                } else {
                    if (self.cursor_pos.index % self.view.columns as usize)
                        != (self.view.columns as usize - 1)
                        && self.cursor_pos.index + 1 < self.file.length
                    {
                        self.cursor_pos.index += 1;
                        self.cursor_pos.first_half = true;
                    }
                }
            }
            Direction::Down => {
                if (self.cursor_pos.index + self.view.columns as usize) < self.file.length {
                    self.cursor_pos.index += self.view.columns as usize;
                }
            }
            Direction::None => {}
        }
        let mut hex_x = (self.cursor_pos.index as u16 % self.view.columns) * 3;
        if !self.cursor_pos.first_half {
            hex_x += 1;
        }

        let old_char_x = old_position.index as u16 % self.view.columns;
        let old_char_y = (old_position.index as u16 / self.view.columns) - self.view.starting_row;
        let char_x = self.cursor_pos.index as u16 % self.view.columns;
        let y = (self.cursor_pos.index as u16 / self.view.columns) - self.view.starting_row;
        let data = self.file.get_data();

        queue!(
            stdout(),
            self.char_window.move_to(old_char_x, old_char_y),
            Print(self.file.get_data()[old_position.index].char()),
            self.char_window.move_to(char_x, y),
            PrintStyledContent(data[self.cursor_pos.index].char().on_white().negative()),
            self.hex_window.move_to(hex_x, y)
        )
        .unwrap();
    }

    fn draw_hex(&self) -> Result<()> {
        let start_point = self.view.starting_row * self.view.columns;
        let end_point =
            (start_point + (self.view.rows * self.view.columns)).min(self.file.length as u16);
        let data = self.file.get_data();
        for y in (start_point..end_point).step_by(self.view.columns as usize) {
            let mut line = String::new();
            for x in 0..self.view.columns {
                if (x + y) < data.len() as u16 {
                    line.push_str(&format!("{} ", data[(y + x) as usize]));
                } else {
                    line.push_str("  ")
                }
            }
            queue!(
                stdout(),
                self.hex_window
                    .move_to(0, (y - start_point) / self.view.columns),
                Print(line)
            )?
        }
        Ok(())
    }

    fn draw_char(&self) -> Result<()> {
        let start_point = self.view.starting_row * self.view.columns;
        let end_point =
            (start_point + (self.view.rows * self.view.columns)).min(self.file.length as u16);
        let data = self.file.get_data();

        for y in (start_point..end_point).step_by(self.view.columns as usize) {
            let mut line = String::new();
            for x in 0..self.view.columns {
                if (x + y) < data.len() as u16 {
                    line.push_str(&data[(y + x) as usize].char());
                } else {
                    line.push_str(" ")
                }
            }
            queue!(
                stdout(),
                self.char_window
                    .move_to(0, (y - start_point) / self.view.columns),
                Print(line)
            )?
        }

        Ok(())
    }

    fn mainloop(&mut self) -> Result<()> {
        self.draw_offset()?;
        self.draw_hex()?;
        self.draw_char()?;
        queue!(stdout(), self.hex_window.move_to(0, 0))?;
        stdout().flush()?;

        'mainloop: loop {
            // `poll()` waits for an `Event` for a given time period
            if poll(Duration::from_millis(500))? {
                // It's guaranteed that the `read()` won't block when the `poll()`
                // function returns `true`
                match read()? {
                    Event::Key(event) => {
                        if event.code == KeyCode::Char('q') {
                            break 'mainloop;
                        } else if event.code == KeyCode::Char('l') {
                            self.move_cursor(Direction::Right)
                        } else if event.code == KeyCode::Char('k') {
                            self.move_cursor(Direction::Up)
                        } else if event.code == KeyCode::Char('j') {
                            self.move_cursor(Direction::Down)
                        } else if event.code == KeyCode::Char('h') {
                            self.move_cursor(Direction::Left)
                        } else if event.code == KeyCode::Down {
                            self.scroll_up()?
                        } else if event.code == KeyCode::Up {
                            self.scroll_down()?
                        }
                    }
                    Event::Resize(columns, rows) => {
                        self.resize(rows, columns);
                    }
                    _ => {}
                }

                stdout().flush()?;
            } else {
                // Timeout expired and no `Event` is available
            }
        }
        Ok(())
    }
}

pub fn run(filepath: String) -> Result<()> {
    let file = HexFile::load_from_file(filepath).expect("File not found");
    let mut app = App::new(file);
    app.mainloop()
}
