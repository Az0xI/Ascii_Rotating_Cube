// Import all the crate
use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
    tty::IsTty,
};
use std::io::stdout;

mod draw;
mod face;
mod point;

// ascii brightness = " .:-=+*#%@"
const SHADOW: [u32; 10] = [
    ' ' as u32, '.' as u32, ':' as u32, '-' as u32, '=' as u32, '+' as u32, '*' as u32, '#' as u32,
    '%' as u32, '@' as u32,
];

// main function
fn main() -> std::io::Result<()> {
    if !stdout().is_tty() {
        println!("The program must be launched in a terminal in order to work");
        return Ok(());
    }

    // Init terminal
    execute!(
        stdout(),
        EnterAlternateScreen,
        Hide,
        SetTitle("Rotating_Cube"),
    )?;
    enable_raw_mode()?;

    draw::axis()?;
    loop {
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
            _ => (),
        };
    }

    // Reset terminal
    execute!(stdout(), LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}
