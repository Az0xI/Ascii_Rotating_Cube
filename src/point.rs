mod point {
    const CENTER: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[derive(Debug)]
    pub struct Point {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Point {
        fn new() -> Point {
            let point = Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            return point;
        }
    }
}

fn main() {
    let mut new_point = point::Point::new();
    println!(
        "x = {}, y = {}, z = {}",
        new_point.x, new_point.y, new_point.z
    );
}
