// Import all the crate
use ratatui;
use crossterm::event;

// ascii brightness = " .:-=+*#%@"
const SHADOW: [u32; 10] = [
    ' ' as u32, '.' as u32, ':' as u32, '-' as u32, '=' as u32, '+' as u32, '*' as u32, '#' as u32,
    '%' as u32, '@' as u32,
];

#[derive(Debug)]
struct Face {
    ul: (i32, i32),
    ur: (i32, i32),
    dl: (i32, i32),
    dr: (i32, i32),
    len: i32,
}

impl Face {
    fn create(size: i32) -> Face {
        let mid = size / 2;
        let new_face: Face = Face {
            ul: (mid, mid),
            ur: (mid, -mid),
            dl: (-mid, mid),
            dr: (-mid, -mid),
            len: size,
        };
        return new_face;
    }
}

// main function
fn main() -> () {
    // Init terminal
    let terminal = ratatui::init();
    let result = game_loop(terminal);
    let face = Face::create(5);

    //draw_axis(center);
    //draw_point(&face, center);
    //draw_face(&face, (center.1 - face.ul.0, center.0 - face.ul.1));
    ratatui::restore();
    result
}

fn game_loop(mut terminal: ratatui::DefaultTerminal) {
    loop {
        terminal.draw(render);
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
/*fn draw_point(face: &Face, pos: (i32, i32)) {
    addch('A' as u32);
}

fn draw_face(face: &Face, pos: (i32, i32)) {
    let mut x = 0;
    let mut y = 0;

    if face.len % 2 == 0 {
        mv(pos.0, pos.1 - face.len / 2 + 1);
    } else {
        mv(pos.0, pos.1 - face.len / 2);
    }
    for _i in 0..face.len {
        for _j in 0..face.len * 2 - 1 {
            addch(SHADOW[9]);
        }
        getyx(stdscr(), &mut y, &mut x);
        if face.len % 2 == 0 {
            mv(y + 1, pos.1 - face.len / 2 + 1);
        } else {
            mv(y + 1, pos.1 - face.len / 2);
        }
    }
}
;8u
fn draw_axis(center: (i32, i32)) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut i = 0;
    let mut j = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    mv(0, center.0);
    while i != max_y {
        addch('|' as u32);
        i += 1;
        mv(i, center.0);
    }
    mv(center.1, 0);
    while j != max_x {
        addch('-' as u32);
        j += 1;
        mv(center.1, j);
    }
    mv(center.1, center.0);
    addch('+' as u32);
    mv(0, 0);
}
 */
