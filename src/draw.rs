use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveTo},
    queue,
    style::Print,
    terminal::size,
};
use std::io::{stdout, Write};

struct TermInfo {
    size: (u16, u16),
    center: (u16, u16),
}

impl TermInfo {
    pub fn new() -> TermInfo {
        let size = match size() {
            Ok(v) => v,
            Err(_e) => panic!(),
        };
        let term = TermInfo {
            size: size,
            center: (size.0 / 2, size.1 / 2),
        };
        term
    }
}

fn normalize_coo(coo: (u16, u16), info: TermInfo) -> (u16, u16) {
    let new_coo: (u16, u16) = (coo.0 + info.center.0, coo.1 + info.center.1);

    new_coo
}

pub fn axis() -> std::io::Result<()> {
    let info = TermInfo::new();

    queue!(stdout(), MoveTo(info.center.0, 0))?;
    for _i in 0..info.size.1 {
        queue!(stdout(), Print('|'), MoveDown(1), MoveLeft(1))?;
    }
    queue!(stdout(), MoveTo(0, info.center.1))?;
    for _j in 0..info.size.0 {
        queue!(stdout(), Print('-'))?;
    }
    queue!(
        stdout(),
        MoveTo(info.center.0, info.center.1),
        Print('+'),
        MoveTo(0, 0)
    )?;
    stdout().flush()?;
    Ok(())
}
