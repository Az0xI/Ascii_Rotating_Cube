pub struct Face {
    ul: (i32, i32),
    ur: (i32, i32),
    dl: (i32, i32),
    dr: (i32, i32),
    len: i32,
}

impl Face {
    pub fn new(size: i32) -> Face {
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
