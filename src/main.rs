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
const SHADOW: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

// main function
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "-h" || args[1] == "--help"{
        println!("DESCRIPTION:\nThis programm is a 3D simulatio inside a TTY.\n");
        println!("USAGE:\n\"-h\",\"--Help\": Print the Help Menu.\n");
        println!("[In Program]\n'h': Display the Help Menu");
        return Ok(())
    }
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
