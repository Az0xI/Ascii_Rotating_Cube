// Inport the crate ncurses
extern crate ncurses;

// Import all ncurses function as it's own (without calling int in a struct)
use ncurses::*;

// ascii brightness = " .:-=+*#%@"
const SHADOW: [u32; 10] = [
    ' ' as u32, '.' as u32, ':' as u32, '-' as u32, '=' as u32,
    '+' as u32, '*' as u32, '#' as u32, '%' as u32, '@' as u32,
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
    //Init Ncurses Window
    initscr();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let center = ((COLS() - 1) / 2 + 1, (LINES() - 1) / 2 + 1);
    let mut face = Face::create(6);

    draw_axis(center);
    draw_face(&face, (center.1 - face.ul.0, center.0 - face.ul.1));
    refresh();
    getch();
    endwin();
}

fn draw_face(face: &Face, pos: (i32, i32)) {
    let mut x = 0; let mut y = 0;

    mv(pos.0, pos.1);
    for _i in 0..face.len {
        for _j in 0..face.len {
            addch(SHADOW[9]);
        }
        getyx(stdscr(), &mut y, &mut x);
        mv(y + 1, pos.1);
    }
}

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
}
