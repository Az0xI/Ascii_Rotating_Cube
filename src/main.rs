// Import all the crate
use crossterm::{
    cursor::{Hide, MoveDown, MoveLeft, MoveTo, Show},
    event::{poll, read},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use std::io::{stdout, Write};

mod face; mod draw;
mod point;

// ascii brightness = " .:-=+*#%@"
const SHADOW: [u32; 10] = [
    ' ' as u32, '.' as u32, ':' as u32, '-' as u32, '=' as u32, '+' as u32, '*' as u32, '#' as u32,
    '%' as u32, '@' as u32,
];

// main function
fn main() -> std::io::Result<()> {
    // Init terminal
    execute!(
        stdout(),
        EnterAlternateScreen,
        Hide,
        SetTitle("Rotating_Cube"),
    )?;
    let point = point::Point::new(0.0, 0.0, 0.0);
    draw::axis()?;
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Reset terminal
    execute!(stdout(), LeaveAlternateScreen, Show)?;
    Ok(())
}
