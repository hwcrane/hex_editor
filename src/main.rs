// fn main() {
//     // let input = vec![
//     //     0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
//     //     0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x44, 0x08, 0x02, 0x00, 0x00, 0x00,
//     // ];
//
//     hex_editor::run("small.png".to_string());
// }

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

use std::io::stdout;

fn main() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    // hex_editor::run("small.png".to_string())?;
    hex_editor::run("IMG_2450.PNG".to_string())?;

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
