use std::fmt;
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
pub fn test_point2d() {
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display:{}", point);
    println!("Debug:{:?}", point);
}
