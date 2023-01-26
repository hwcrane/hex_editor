use crate::hex_file::HexFile;

mod byte;
mod hex_file;

use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, PrintStyledContent, Stylize};
use crossterm::{
    event::{poll, read, Event, KeyCode},
    style, Result,
};

fn print_data(file: &HexFile) -> Result<()> {
    let data = file.get_data();
    let columns = 24;

    for i in (0..data.len()).step_by(columns) {
        // Print offset
        queue!(
            stdout(),
            MoveTo( 0,(i / columns) as u16),
            PrintStyledContent(format!("{:08X} ", i).black())
        )?;

        // Print Hex data
        for ii in 0..columns {
            if data.len() > (i + ii) {
                queue!(stdout(), Print(format!("{} ", data[i + ii])))?;
                // print!("{} ", data[i + ii])
            } else {
                queue!(stdout(), Print("   "))?;
                // print!("   ")
            }
        }

        queue!(stdout(), Print(" "))?;
        // print!(" ");

        // Print char data
        for ii in 0..columns {
            if data.len() > (i + ii) {
                queue!(stdout(), Print(format!("{}", data[i + ii].char())))?;
                // print!("{}", data[i + ii].char())
            } else {
                queue!(stdout(), Print(" "))?;
                // print!(" ")
            }
        }
    }
    stdout().flush()?;
    Ok(())
}

pub fn run(filepath: String) -> Result<()> {
    let file = HexFile::load_from_file(filepath).expect("File not found");
    print_data(&file)?;
    // file.print();
    'mainloop: loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break 'mainloop;
                    }
                }
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
    Ok(())
}
